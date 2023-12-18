#!/bin/bash

cargo build

export LD_LIBRARY_PATH=/home/breno/Desktop/Projects/brado/brado-c/target/debug:$LD_LIBRARY_PATH

cbindgen --config cbindgen.toml --crate brado-c  --output include/brado.h

g++ --std=c++11 -o target/test examples/test.cpp -Ltarget/debug/ -lbrado

./target/test
