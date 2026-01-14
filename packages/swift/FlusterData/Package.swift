// swift-tools-version: 6.2
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let package = Package(
  name: "FlusterData",
  platforms: [
    .iOS(.v26),
    .macOS(.v26)
  ],
  products: [
    // Products define the executables and libraries a package produces, making them visible to other packages.
    .library(
      name: "FlusterData",
      targets: ["FlusterData"]
    )
  ],
  dependencies: [
    .package(path: "../../rust/fluster_swift_mdx_parser/FlusterSwiftMdxParser/"),
    .package(path: "../FlusterMdx/"),
    .package(
      url: "https://github.com/MaxHaertwig/SwiftyBibtex.git",
      .upToNextMajor(from: "1.0.0")
    ),
    .package(
      url: "https://github.com/google/flatbuffers.git",
      exact: "25.9.23"
    ),
    .package(
      url: "https://github.com/swiftlang/swift-testing",
      .upToNextMajor(from: "6.2.3")
    )
  ],
  targets: [
    // Targets are the basic building blocks of a package, defining a module or a test suite.
    // Targets can depend on other targets in this package and products from dependencies.
    .target(
      name: "FlusterData",
      dependencies: [
        "SwiftyBibtex",
        .product(name: "FlusterSwiftMdxParser", package: "FlusterSwiftMdxParser"),
        .product(name: "FlusterMdx", package: "FlusterMdx"),
        .product(name: "FlatBuffers", package: "flatbuffers")
      ]
    ),
    .testTarget(
      name: "FlusterDataTests",
      dependencies: ["FlusterData", "FlusterSwiftMdxParser", "FlusterMdx"]
    )
  ],
)
