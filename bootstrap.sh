#!/bin/bash

mkdir ./crashes
mkdir ./fuzz_inputs
mkdir ./corpus

cd fuzz_target
./make.sh
