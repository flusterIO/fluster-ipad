// swift-tools-version: 6.2
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let package = Package(
  name: "FlusterMdx",
  platforms: [
    .iOS(.v26),
    .macOS(.v15)
  ],
  products: [
    // Products define the executables and libraries a package produces, making them visible to other packages.
    .library(
      name: "FlusterMdx",
      targets: ["FlusterMdx"]
    )
  ],
  dependencies: [
    .package(path: "../../rust/fluster_swift_mdx_parser/FlusterSwiftMdxParser/"),
    .package(
      url: "https://github.com/swiftlang/swift-testing",
      .upToNextMajor(from: "6.2.3")
    ),
  ],
  targets: [
    // Targets are the basic building blocks of a package, defining a module or a test suite.
    // Targets can depend on other targets in this package and products from dependencies.
    .target(
      name: "FlusterMdx",
      dependencies: [
        .product(name: "FlusterSwiftMdxParser", package: "FlusterSwiftMdxParser")
      ]
    ),
    .testTarget(
      name: "FlusterMdxTests",
      dependencies: ["FlusterMdx", "FlusterSwiftMdxParser"]
    )
  ],
)
