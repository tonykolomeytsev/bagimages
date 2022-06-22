#!/bin/bash

cleanup() {
  printf "Cleaning up... "
  rm -rf bagimages-x86_64-unknown-linux-musl.tar.gz
  rm -rf dist
  echo "Done"
}

trap 'cleanup' EXIT

echo "Downloading... "
sudo curl -LJO https://github.com/tonykolomeytsev/bagimages/releases/latest/download/bagimages-x86_64-unknown-linux-musl.tar.gz
echo "Unpacking... "
tar -xzvf bagimages-x86_64-unknown-linux-musl.tar.gz
sudo mv ./dist/bagimages /usr/local/bin/bagimages
echo "Installed"