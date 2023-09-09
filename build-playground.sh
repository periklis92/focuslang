#!/bin/bash

ROOT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
PLAYGROUND_DIR=$ROOT_DIR/playground
BUILD_DIR=$PLAYGROUND_DIR/build

npm run build --prefix $PLAYGROUND_DIR

cp $BUILD_DIR/index.html .
cp -r $BUILD_DIR/playground-app ./playground-app