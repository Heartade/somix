#!/usr/bin/env sh

rustup toolchain install stable && rustup target add wasm32-unknown-unknown
bash -cl "wget -qO- https://github.com/thedodd/trunk/releases/download/v0.16.0/trunk-x86_64-unknown-linux-gnu.tar.gz | tar -xzf- && mv ./trunk /usr/bin/"
trunk build $@