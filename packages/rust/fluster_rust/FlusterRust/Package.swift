// swift-tools-version:5.5
// The swift-tools-version declares the minimum version of Swift required to build this package.
// Swift Package: FlusterRust

import PackageDescription;

let package = Package(
    name: "FlusterRust",
    platforms: [
        .iOS(.v13),
        .macOS(.v10_15)
    ],
    products: [
        .library(
            name: "FlusterRust",
            targets: ["FlusterRust"]
        )
    ],
    dependencies: [ ],
    targets: [
        .binaryTarget(name: "RustFramework", path: "./RustFramework.xcframework"),
        .target(
            name: "FlusterRust",
            dependencies: [
                .target(name: "RustFramework")
            ]
        ),
    ]
)