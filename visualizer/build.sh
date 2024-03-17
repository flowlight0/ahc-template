#!/bin/sh

set -ex

wasm-pack build --target web
mkdir -p www
rm -rf www/*
cp index.html www/
cp gif.worker.* www/
cp -r js www/
cp pkg/*.js www/
cp pkg/*.wasm www/
cd www
python3 -m http.server
