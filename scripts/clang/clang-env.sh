#!/usr/bin/env bash
DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
VERSION=3.4.2
CLANG_PATH="$DIR/modules/clang/$VERSION"

export PATH=$CLANG_PATH/bin:$PATH
export CPATH=$CLANG_PATH/include:$CPATH
export LIBRARY_PATH=$CLANG_PATH/lib:$LIBRARY_PATH
export LD_LIBRARY_PATH=$CLANG_PATH/lib:$LD_LIBRARY_PATH
export MANPATH=$CLANG_PATH/share/man:$MANPATH
export PKG_CONFIG_PATH=$CLANG_PATH/lib/pkgconfig:$PKG_CONFIG_PATH
