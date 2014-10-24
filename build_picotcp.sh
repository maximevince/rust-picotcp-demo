#!/bin/bash
PICOTCP=~/picotcp
rm -rf $PWD/build
mkdir -p $PWD/build && make -C $PICOTCP TUN=1 TAP=1 ARCH=shared PREFIX=$PWD/build
mkdir -p $PWD/target/deps
cp -a $PWD/build/lib/libpicotcp.a .
cp $PWD/build/lib/libpicotcp.a $PWD/target/deps

