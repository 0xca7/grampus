#!/bin/bash

mkdir ./crashes
mkdir ./fuzz_inputs

cd fuzz_target
./make.sh
