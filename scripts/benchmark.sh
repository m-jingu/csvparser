#!/bin/bash
# CSV Parser Benchmark Script
# Comprehensive performance testing for the CSV parser

set -e

# Colorized output functions
print_header() {
    echo -e "\n\033[1;34m=== $1 ===\033[0m"
}

print_success() {
    echo -e "\033[1;32m✓ $1\033[0m"
}

print_error() {
    echo -e "\033[1;31m✗ $1\033[0m"
}

print_info() {
    echo -e "\033[1;33mℹ $1\033[0m"
}

# Configuration
RUST_BINARY="./target/release/csvparser"
PYTHON_SCRIPT="./csvparser_pd.py"
TEST_FILES=(
    "test_small.csv:1000:5"
    "test_medium.csv:100000:10"
    "test_large.csv:1000000:20"
)

# Generate test files
generate_test_files() {
    print_header "Test File Generation"
    
    for file_spec in "${TEST_FILES[@]}"; do
        IFS=':' read -r filename rows cols <<< "$file_spec"
        
        if [ ! -f "$filename" ]; then
            print_info "Generating $filename ($rows rows, $cols columns)..."
            python3 scripts/generate_test_data.py --rows "$rows" --cols "$cols" --output "$filename"
            print_success "Generated $filename"
        else
            print_info "$filename already exists, skipping..."
        fi
    done
}

# Build Rust version
build_rust() {
    print_header "Rust Build"
    
    if [ ! -f "$RUST_BINARY" ]; then
        print_info "Building Rust version..."
        cargo build --release
        print_success "Rust build completed"
    else
        print_info "Rust binary already exists, skipping build..."
    fi
}

# Run benchmark tests
run_benchmark() {
    local test_file="$1"
    local description="$2"
    
    print_header "Benchmark: $description"
    
    # Get file size
    local file_size=$(du -h "$test_file" | cut -f1)
    print_info "File size: $file_size"
    
    # Rust version benchmark
    print_info "Testing Rust version..."
    local rust_start=$(date +%s.%N)
    "$RUST_BINARY" -i "$test_file" --stats > /dev/null
    local rust_end=$(date +%s.%N)
    local rust_time=$(echo "$rust_end - $rust_start" | bc)
    
    # Python version benchmark (if available)
    if [ -f "$PYTHON_SCRIPT" ]; then
        print_info "Testing Python version..."
        local python_start=$(date +%s.%N)
        python3 "$PYTHON_SCRIPT" -f 0 "$test_file" > /dev/null
        local python_end=$(date +%s.%N)
        local python_time=$(echo "$python_end - $python_start" | bc)
        
        # Compare results
        local speedup=$(echo "scale=2; $python_time / $rust_time" | bc)
        print_success "Rust: ${rust_time}s, Python: ${python_time}s (${speedup}x faster)"
    else
        print_success "Rust: ${rust_time}s"
    fi
}

# Memory usage test
test_memory_usage() {
    print_header "Memory Usage Test"
    
    local test_file="test_large.csv"
    if [ ! -f "$test_file" ]; then
        print_error "Test file $test_file not found"
        return 1
    fi
    
    print_info "Monitoring memory usage during processing..."
    
    # Start memory monitoring in background
    (while true; do
        ps aux | grep csvparser | grep -v grep | awk '{print $6}' | head -1
        sleep 0.1
    done) > memory_usage.log &
    local monitor_pid=$!
    
    # Execute CSV processing
    "$RUST_BINARY" -i "$test_file" --stats > /dev/null
    
    # Stop monitoring
    kill $monitor_pid 2>/dev/null || true
    
    # Calculate maximum memory usage
    local max_memory=$(sort -n memory_usage.log | tail -1)
    local max_memory_mb=$((max_memory / 1024))
    
    print_success "Maximum memory usage: ${max_memory_mb} MB"
    
    # Remove log file
    rm -f memory_usage.log
}

# Field selection test
test_field_selection() {
    print_header "Field Selection Test"
    
    local test_file="test_medium.csv"
    if [ ! -f "$test_file" ]; then
        print_error "Test file $test_file not found"
        return 1
    fi
    
    print_info "Testing field selection (columns 1, 3, 5)..."
    
    local start_time=$(date +%s.%N)
    "$RUST_BINARY" -i "$test_file" -f 1,3,5 --stats > selected_output.csv
    local end_time=$(date +%s.%N)
    local processing_time=$(echo "$end_time - $start_time" | bc)
    
    # Verify output file line count
    local output_lines=$(wc -l < selected_output.csv)
    local input_lines=$(wc -l < "$test_file")
    
    print_success "Field selection completed in ${processing_time}s"
    print_success "Input lines: $input_lines, Output lines: $output_lines"
    
    # Remove output file
    rm -f selected_output.csv
}

# Large file test (optional)
test_large_file() {
    print_header "Large File Test"
    
    local large_file="test_10gb.csv"
    local target_size_gb=10
    
    print_info "This will create a ${target_size_gb}GB test file. Continue? (y/N)"
    read -r response
    if [[ "$response" != "y" && "$response" != "Y" ]]; then
        print_info "Skipping large file test"
        return 0
    fi
    
    # Generate large file
    print_info "Generating ${target_size_gb}GB test file (this may take a while)..."
    python3 scripts/generate_test_data.py --size-gb "$target_size_gb" --cols 15 --output "$large_file"
    
    if [ -f "$large_file" ]; then
        print_info "Testing with large file..."
        local start_time=$(date +%s.%N)
        "$RUST_BINARY" -i "$large_file" --stats --buffer-size 16777216 > /dev/null
        local end_time=$(date +%s.%N)
        local processing_time=$(echo "$end_time - $start_time" | bc)
        
        print_success "Large file processing completed in ${processing_time}s"
        
        # Remove large file (optional)
        print_info "Remove large test file? (y/N)"
        read -r response
        if [[ "$response" == "y" || "$response" == "Y" ]]; then
            rm -f "$large_file"
            print_success "Large test file removed"
        fi
    fi
}

# Main execution
main() {
    print_header "CSV Parser Benchmark Start"
    
    # Check required tools
    if ! command -v bc &> /dev/null; then
        print_error "bc command not found. Please install bc."
        exit 1
    fi
    
    if ! command -v python3 &> /dev/null; then
        print_error "python3 not found. Please install Python 3."
        exit 1
    fi
    
    # Generate test files
    generate_test_files
    
    # Build Rust version
    build_rust
    
    # Run benchmarks on each test file
    for file_spec in "${TEST_FILES[@]}"; do
        IFS=':' read -r filename rows cols <<< "$file_spec"
        if [ -f "$filename" ]; then
            run_benchmark "$filename" "$filename ($rows rows, $cols columns)"
        fi
    done
    
    # Additional tests
    test_memory_usage
    test_field_selection
    
    # Large file test (optional)
    test_large_file
    
    print_header "Benchmark Complete"
    print_success "All tests completed successfully!"
}

# Execute script
main "$@"
