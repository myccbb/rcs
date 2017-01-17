#!/bin/env python

import os, sys

# TODO print error message to stderr

def addpath():
    if len(sys.argv) != 4:
        return ""
    env_name = sys.argv[2]
    path = sys.argv[3]

    env_value = os.getenv(env_name)
    if env_value is None:
        return ""

    envs = [env for env in env_value.split(':') if env != '' and env != path ]
    env_str = ':'.join(envs)
    if env_str is '':
        env_str = path
    else:
        env_str = path + ":" + env_str
    return env_str

if __name__ == '__main__':
    if len(sys.argv) < 2:
        print
    elif sys.argv[1] == "addpath":
        print addpath()
