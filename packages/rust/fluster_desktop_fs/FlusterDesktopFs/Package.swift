// swift-tools-version:5.5
// The swift-tools-version declares the minimum version of Swift required to build this package.
// Swift Package: FlusterDesktopFs

import PackageDescription;

let package = Package(
    name: "FlusterDesktopFs",
    platforms: [
        .iOS(.v13),
        .macOS(.v10_15)
    ],
    products: [
        .library(
            name: "FlusterDesktopFs",
            targets: ["FlusterDesktopFs"]
        )
    ],
    dependencies: [ ],
    targets: [
        .binaryTarget(name: "RustFramework", path: "./RustFramework.xcframework"),
        .target(
            name: "FlusterDesktopFs",
            dependencies: [
                .target(name: "RustFramework")
            ]
        ),
    ]
)