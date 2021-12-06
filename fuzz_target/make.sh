#!/bin/bash
rm example_target
gcc -Wall example_target.c -o example_target --include kson.h kson.c
