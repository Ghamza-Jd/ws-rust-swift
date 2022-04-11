// swift-tools-version:5.5

import PackageDescription
import Foundation

let package = Package(
        name: "WsRustSwift",
        platforms: [
            .iOS(.v13)
        ],
        products: [
            .library(
                name: "WsRustSwift",
                targets: ["WsRustSwift"]
            ),
        ],
        targets: [
            .target(
                name: "WsRustSwift",
                dependencies: ["WsRustSwift"]
            ),
            .binaryTarget(
                name: "WsRust",
                url: "https://github.com/Ghamza-Jd/ws-rust-swift",
                // todo: automate the checksum assignment
                checksum: "3733e24fa44f28bf31daa8b94037279c56d823028eb1a9ce09a1d03a355642a8"
            ),
        ]
)