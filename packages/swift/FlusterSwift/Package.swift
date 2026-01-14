// swift-tools-version: 6.2
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let package = Package(
  name: "FlusterSwift",
  platforms: [
    .iOS(.v26),
    .macOS(.v26)
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
    .package(path: "../../rust/fluster_swift_mdx_parser/FlusterSwiftMdxParser/"),
    .package(path: "../FlusterData"),
    .package(
      url: "https://github.com/google/flatbuffers.git",
      from: "25.9.23"
    ),
    .package(
      url: "https://github.com/swiftlang/swift-testing",
      .upToNextMajor(from: "6.2.3")
    ),
    .package(url: "https://github.com/elai950/AlertToast", .upToNextMajor(from: "1.3.9")),
  ],
  targets: [
    // Targets are the basic building blocks of a package, defining a module or a test suite.
    // Targets can depend on other targets in this package and products from dependencies.
    .target(
      name: "FlusterSwift",
      dependencies: [
        "SwiftyBibtex",
        .product(name: "FlusterSwiftMdxParser", package: "FlusterSwiftMdxParser"),
        .product(name: "FlusterData", package: "FlusterData"),
        .product(name: "FlatBuffers", package: "flatbuffers"),
        .product(name: "AlertToast", package: "AlertToast")
      ]
    ),
    .testTarget(
      name: "FlusterSwiftTests",
      dependencies: ["FlusterSwift", "FlusterSwiftMdxParser", "FlusterData"]
    )
  ],
)
