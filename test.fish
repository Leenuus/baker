#!/bin/fish

cargo build 2>&1 >/dev/null
./target/debug/baker add ./.bashrc
./target/debug/baker add ../testrc
./target/debug/baker add ./dot_rc

set baker $HOME/.local/baker
python ./test.py
cd $baker
tree

# clean up
rm $baker/index
rm -rf $baker/repo/*
