// swift-tools-version: 6.2
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let package = Package(
  name: "FlusterWebviewClients",
  platforms: [
    .iOS(.v26),
    .macOS(.v26)
  ],
  products: [
    // Products define the executables and libraries a package produces, making them visible to other packages.
    .library(
      name: "FlusterWebviewClients",
      targets: ["FlusterWebviewClients"]
    )
  ],
  dependencies: [
    .package(path: "../FlusterData"),
    .package(path: "../FlusterMdx"),
    .package(
      url: "https://github.com/swiftlang/swift-testing",
      .upToNextMajor(from: "6.2.3")
    ),
  ],
  targets: [
    // Targets are the basic building blocks of a package, defining a module or a test suite.
    // Targets can depend on other targets in this package and products from dependencies.
    .target(
      name: "FlusterWebviewClients",
      dependencies: [
        .product(name: "FlusterData", package: "FlusterData"),
        .product(name: "FlusterMdx", package: "FlusterMdx")
      ]
    ),
    .testTarget(
      name: "FlusterSwiftTests",
      dependencies: ["FlusterData", "FlusterMdx"]
    )
  ],
)
