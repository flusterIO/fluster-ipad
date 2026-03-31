// swift-tools-version: 6.2
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let package = Package(
  name: "FlusterAI",
  platforms: [
    .iOS(.v26),
    .macOS(.v26)
  ],
  products: [
    // Products define the executables and libraries a package produces, making them visible to other packages.
    .library(
      name: "FlusterAI",
      targets: ["FlusterAI"]
    )
  ],
  dependencies: [
    .package(path: "../FlusterData"),
    .package(path: "../../rust/conundrum_swift/ConundrumSwift/"),
  ],
  targets: [
    // Targets are the basic building blocks of a package, defining a module or a test suite.
    // Targets can depend on other targets in this package and products from dependencies.
    .target(
      name: "FlusterAI",
      dependencies: [
        .product(name: "ConundrumSwift", package: "ConundrumSwift"),
        .product(name: "FlusterData", package: "FlusterData")
      ]
    ),
    .testTarget(
      name: "FlusterAITests",
      dependencies: ["FlusterAI"]
    )
  ]
)
