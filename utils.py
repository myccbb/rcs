#!/usr/bin/env python
# encoding: utf-8

from __future__ import print_function
import os, sys, signal

def eprint(*args):
    print(*args, file=sys.stderr)

def remove_dup_path_item(env_value):
    rawlist = env_value.split(':')
    list = [i for i in rawlist if i != '']
    list.reverse()

    new_list = []
    pos = 0
    for item in list:
        pos += 1
        # print(item, list[pos:])
        if item not in list[pos:]:
            new_list.append(item)
    new_list.reverse()
    return ':'.join(new_list)

def addpath(envname, path):
    env_value = os.getenv(envname)
    if env_value is None:
        # eprint('env', envname, 'not found!')
        return path
    return remove_dup_path_item(path + ':' + os.getenv(envname))


def ignore_handler():
    pass

def endless_run(cmd: str):
    signal.signal(signal.SIGINT, ignore_handler)
    while True:
        os.system(cmd)

if '__main__' == __name__:
    argv = sys.argv
    if len(argv) < 2:
        eprint('need parameter!')
        exit(1)
    if argv[1] == '--addpath':
        if len(argv) != 4:
            eprint(argv)
            eprint('Usage:', argv[0], argv[1], '<env name> <path>')
            exit(1)
        print(addpath(argv[2], argv[3]))
    elif argv[1] == '--endless-run':
        if len(argv) < 3:
            eprint('Usage:', argv[0], argv[1], '<cmd>')
            exit(1)
        endless_run(' '.join(argv[2:]))
    else:
        eprint('unknown command:', argv[1])
        exit(1)
