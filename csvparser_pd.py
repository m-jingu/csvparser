#!/usr/bin/env python

import sys
import argparse
import pandas as pd

version = '%(prog)s 20160810'

def ArgParse():
    parser = argparse.ArgumentParser(description=
    '''This script parse csv data.

    Create Date: 2016-08-10 ''',
    formatter_class=argparse.RawDescriptionHelpFormatter)

    parser.add_argument('infile', nargs='?', type=argparse.FileType('rU'), metavar='FILE', help='CSV File', default=sys.stdin)
    parser.add_argument('-f', '--fields', action='store', type=str, metavar='LIST', help='select only these fields')
    parser.add_argument('-v', '--version', action='version', version=version)
    args = parser.parse_args()

    return args

def dumpCsv(infile, fields):
    try:
        df = pd.read_csv(infile, usecols=fields)
    except:
        raise
    print(df.to_csv(index=False)[:-1])

if __name__ == "__main__":
    args = ArgParse()
    if args.fields == None or args.fields == '0':
        fields = None
    else:
        fields = map(int, args.fields.split(','))
        fields = [i - 1 for i in fields]
    dumpCsv(args.infile, fields)

