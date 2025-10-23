#!/usr/bin/env python3
"""
Large CSV test data generation script
Capable of generating test data up to 100GB
"""

import csv
import sys
import argparse
import os
from datetime import datetime, timedelta
import random
import string

def generate_random_string(length=10):
    """Generate a random string of specified length"""
    return ''.join(random.choices(string.ascii_letters + string.digits, k=length))

def generate_random_date():
    """Generate a random date between 2020-01-01 and 2024-12-31"""
    start_date = datetime(2020, 1, 1)
    end_date = datetime(2024, 12, 31)
    time_between = end_date - start_date
    days_between = time_between.days
    random_days = random.randrange(days_between)
    return (start_date + timedelta(days=random_days)).strftime('%Y-%m-%d')

def generate_test_data(num_rows, num_cols, output_file):
    """Generate test data with various data types and edge cases"""
    print(f"Generating {num_rows:,} rows with {num_cols} columns...")
    print(f"Output file: {output_file}")
    
    # Generate headers
    headers = [f"col_{i:03d}" for i in range(num_cols)]
    
    with open(output_file, 'w', newline='', encoding='utf-8') as csvfile:
        writer = csv.writer(csvfile)
        writer.writerow(headers)
        
        # Generate data rows
        for row_num in range(num_rows):
            if row_num % 100000 == 0:
                print(f"Progress: {row_num:,} rows generated")
            
            row = []
            for col in range(num_cols):
                if col == 0:
                    # First column: row number
                    row.append(str(row_num + 1))
                elif col == 1:
                    # Second column: date
                    row.append(generate_random_date())
                elif col == 2:
                    # Third column: integer
                    row.append(str(random.randint(1, 1000000)))
                elif col == 3:
                    # Fourth column: floating point number
                    row.append(f"{random.uniform(0, 1000):.2f}")
                elif col == 4:
                    # Fifth column: string (may contain commas)
                    text = generate_random_string(20)
                    if random.random() < 0.1:  # 10% chance to include comma
                        text = f'"{text}, with comma"'
                    row.append(text)
                elif col == 5:
                    # Sixth column: string (may contain newlines)
                    text = generate_random_string(15)
                    if random.random() < 0.05:  # 5% chance to include newline
                        text = f'"{text}\nwith newline"'
                    row.append(text)
                else:
                    # Other columns: random strings
                    row.append(generate_random_string(15))
            
            writer.writerow(row)
    
    print(f"Generation completed: {num_rows:,} rows written to {output_file}")

def estimate_file_size(num_rows, num_cols):
    """Estimate the file size based on number of rows and columns"""
    # Estimate average bytes per row
    avg_bytes_per_row = num_cols * 20 + 50  # Rough estimate
    total_bytes = num_rows * avg_bytes_per_row
    
    # Convert units
    if total_bytes < 1024:
        return f"{total_bytes} B"
    elif total_bytes < 1024 * 1024:
        return f"{total_bytes / 1024:.1f} KB"
    elif total_bytes < 1024 * 1024 * 1024:
        return f"{total_bytes / (1024 * 1024):.1f} MB"
    else:
        return f"{total_bytes / (1024 * 1024 * 1024):.1f} GB"

def main():
    parser = argparse.ArgumentParser(description='Generate large CSV test data')
    parser.add_argument('--rows', type=int, default=1000000, help='Number of rows to generate (default: 1,000,000)')
    parser.add_argument('--cols', type=int, default=10, help='Number of columns (default: 10)')
    parser.add_argument('--output', type=str, default='test_data.csv', help='Output file name')
    parser.add_argument('--size-gb', type=float, help='Target file size in GB (overrides --rows)')
    
    args = parser.parse_args()
    
    # Calculate number of rows if size is specified
    if args.size_gb:
        # Estimate average bytes per row (depends on number of columns)
        avg_bytes_per_row = args.cols * 20 + 50
        target_bytes = args.size_gb * 1024 * 1024 * 1024
        args.rows = int(target_bytes / avg_bytes_per_row)
        print(f"Calculated {args.rows:,} rows for {args.size_gb} GB file")
    
    # Estimate file size
    estimated_size = estimate_file_size(args.rows, args.cols)
    print(f"Estimated file size: {estimated_size}")
    
    # Confirmation for large files
    if args.rows > 10000000:  # More than 10 million rows
        response = input(f"This will generate {args.rows:,} rows. Continue? (y/N): ")
        if response.lower() != 'y':
            print("Cancelled.")
            return
    
    # Generate test data
    generate_test_data(args.rows, args.cols, args.output)
    
    # Display actual file size
    if os.path.exists(args.output):
        actual_size = os.path.getsize(args.output)
        if actual_size < 1024:
            size_str = f"{actual_size} B"
        elif actual_size < 1024 * 1024:
            size_str = f"{actual_size / 1024:.1f} KB"
        elif actual_size < 1024 * 1024 * 1024:
            size_str = f"{actual_size / (1024 * 1024):.1f} MB"
        else:
            size_str = f"{actual_size / (1024 * 1024 * 1024):.1f} GB"
        
        print(f"Actual file size: {size_str}")

if __name__ == '__main__':
    main()
