#!/bin/bash
# cargo install --force cbindgen
cbindgen --lang c --output ./include/wsrustswift.h

__module="module WsRustSwift {
    header \"wsrustswift.h\"
    export *
}"

echo "$__module" > ./include/module.modulemap

rustup target add aarch64-apple-ios
cargo build --release --target aarch64-apple-ios

rustup target add x86_64-apple-ios
cargo build --release --target x86_64-apple-ios

rustup target add aarch64-apple-ios-sim
cargo build --release --target aarch64-apple-ios-sim

lipo -create \
  target/x86_64-apple-ios/release/libwsrustswift.a \
  target/aarch64-apple-ios-sim/release/libwsrustswift.a \
  -output wsrustswift_iossimulator.a

xcodebuild -create-xcframework \
  -library ./wsrustswift_iossimulator.a \
  -headers ./include/ \
  -library ./target/aarch64-apple-ios/release/libwsrustswift.a \
  -headers ./include/ \
  -output WsRustSwift.xcframework

zip -r bundle.zip WsRustSwift.xcframework

openssl dgst -sha256 bundle.zip