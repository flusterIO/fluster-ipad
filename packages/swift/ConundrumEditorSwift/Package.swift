// swift-tools-version: 6.2
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let package = Package(
    name: "ConundrumEditorSwift",
    products: [
        // Products define the executables and libraries a package produces, making them visible to other packages.
        .library(
            name: "ConundrumEditorSwift",
            targets: ["ConundrumEditorSwift"]
        ),
    ],
    dependencies: [
          .package(url: "https://github.com/Lakr233/libghostty-spm.git", from: "1.0.0"),
    ],
    targets: [
        // Targets are the basic building blocks of a package, defining a module or a test suite.
        // Targets can depend on other targets in this package and products from dependencies.
        .target(
            name: "ConundrumEditorSwift",
            dependencies: [
                .product(name: "GhosttyTerminal", package: "libghostty-spm"),
            ],
        ),
        .testTarget(
            name: "ConundrumEditorSwiftTests",
            dependencies: ["ConundrumEditorSwift"]
        ),
    ]
)
