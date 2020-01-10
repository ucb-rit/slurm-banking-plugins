#!/usr/bin/env bash

MODULE_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SOFTWARE=clang
VERSION=3.4.2

CMAKE_MODULE_DIR=$MODULE_ROOT/modules/cmake/3.16.0-rc3
mkdir -p $CMAKE_MODULE_DIR
cd $CMAKE_MODULE_DIR
if [ ! -d $CMAKE_MODULE_DIR/cmake-3.16.0-rc3-Linux-x86_64 ]; then
    wget https://github.com/Kitware/CMake/releases/download/v3.16.0-rc3/cmake-3.16.0-rc3-Linux-x86_64.tar.gz
    tar -xvf cmake-3.16.0-rc3-Linux-x86_64.tar.gz
fi
export PATH=$(pwd)/cmake-3.16.0-rc3-Linux-x86_64/bin:$PATH

SOURCE_SOFTWARE_DIR=$MODULE_ROOT/sources/$SOFTWARE
SOURCE_DIR=$SOURCE_SOFTWARE_DIR/$VERSION
mkdir -p $SOURCE_DIR

MODULE_SOFTWARE_DIR=$MODULE_ROOT/modules/$SOFTWARE
MODULE_DIR=$MODULE_SOFTWARE_DIR/$VERSION
mkdir -p $MODULE_DIR

cd $SOURCE_DIR

rm -rf llvm-${VERSION}.src.tar.gz
if [ ! -d llvm-${VERSION}.src ]; then
    if [ ! -f llvm-${VERSION}.src.tar.gz ]; then
        wget http://releases.llvm.org/${VERSION}/llvm-${VERSION}.src.tar.gz
    fi
    tar -xvf llvm-${VERSION}.src.tar.gz
fi
cd llvm-${VERSION}.src/tools/
if [ ! -d clang ]; then
    if [ ! -f cfe-${VERSION}.src.tar.gz ]; then
        wget http://releases.llvm.org/${VERSION}/cfe-${VERSION}.src.tar.gz
    fi
    tar -xvf cfe-${VERSION}.src.tar.gz
    mv cfe-${VERSION}.src clang
fi

cd ..
rm -rf build && mkdir -p build && cd build
cmake -DCMAKE_INSTALL_PREFIX=$MODULE_DIR .. || exit 1
make -j2 || exit 1
make install

