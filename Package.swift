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
                checksum: "c58845a196aaa773555d9305b20addde70b272158f6abc202ca6a70d4a20731d"
            ),
        ]
)