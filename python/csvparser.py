#!/usr/bin/env python
# -*- coding: utf-8 -*-

import sys
import argparse
import csv

version = '%(prog)s 20160808'

def ArgParse():
    parser = argparse.ArgumentParser(description=
    '''This script parse csv data.

    Create Date: 2016-08-08 ''',
    formatter_class=argparse.RawDescriptionHelpFormatter)

    parser.add_argument('infile', nargs='?', type=argparse.FileType('rU'), metavar='FILE', help='CSV File', default=sys.stdin)
    parser.add_argument('-f', '--fields', action='store', type=str, metavar='LIST', help='select only these fields')
    parser.add_argument('-v', '--version', action='version', version=version)
    args = parser.parse_args()

    return args

def dumpCsv(infile, fields):
    reader = csv.reader(infile)
    writer = csv.writer(sys.stdout)
    if fields == 0 or fields == ['0']:
        for row in reader:
            writer.writerow(row)
    else:
        for row in reader:
            line = []
            for index, col in enumerate(row):
                for field in fields:
                    if index == int(field) - 1:
                        line.append(col)
            writer.writerow(line)

if __name__ == "__main__":
    args = ArgParse()
    if args.fields == None:
        fields = 0
    else:
        fields = args.fields.split(',')
    dumpCsv(args.infile, fields)


