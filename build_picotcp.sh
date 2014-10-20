#!/bin/bash
PICOTCP=~/picotcp
rm -rf $PWD/build
mkdir -p $PWD/build && TUN=1 ARCH=shared PREFIX=$PWD/build make -C $PICOTCP
cp $PWD/build/lib/libpicotcp.a $PWD/target/deps

