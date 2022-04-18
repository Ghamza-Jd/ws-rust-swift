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
        ]
)