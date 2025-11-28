// swift-tools-version: 6.2
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let package = Package(
    name: "FlusterSwift",
    platforms: [
        .iOS(.v17)
    ],
    products: [
        // Products define the executables and libraries a package produces, making them visible to other packages.
        .library(
            name: "FlusterSwift",
            targets: ["FlusterSwift"]
        )
    ],
    dependencies: [
        .package(
            url: "https://github.com/MaxHaertwig/SwiftyBibtex.git",
            .upToNextMajor(from: "1.0.0")
        ),
        .package(
            url: "https://github.com/gonzalezreal/swift-markdown-ui",
            .upToNextMajor(from: "2.4.1")
        ),
    ],
    targets: [
        // Targets are the basic building blocks of a package, defining a module or a test suite.
        // Targets can depend on other targets in this package and products from dependencies.
        .target(
            name: "FlusterSwift",
            dependencies: [
                "SwiftyBibtex",
                .product(name: "MarkdownUI", package: "swift-markdown-ui"),
            ]
        ),
        .testTarget(
            name: "FlusterSwiftTests",
            dependencies: ["FlusterSwift"]
        ),
    ],
)
