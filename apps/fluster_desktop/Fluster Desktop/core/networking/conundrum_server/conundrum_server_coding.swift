//
//  conundrum_server_coding.swift
//  Fluster Desktop
//

import Foundation

/// The server speaks snake_case JSON. Swift types use camelCase property
/// names and rely on these strategies, so a Swift property must be the exact
/// camelCase form of its wire key (e.g. `respectGitignore` ↔ `respect_gitignore`).
nonisolated enum ConundrumServerCoding {
  static func makeEncoder() -> JSONEncoder {
    let encoder = JSONEncoder()
    encoder.keyEncodingStrategy = .convertToSnakeCase
    return encoder
  }

  static func makeDecoder() -> JSONDecoder {
    let decoder = JSONDecoder()
    decoder.keyDecodingStrategy = .convertFromSnakeCase
    return decoder
  }
}
