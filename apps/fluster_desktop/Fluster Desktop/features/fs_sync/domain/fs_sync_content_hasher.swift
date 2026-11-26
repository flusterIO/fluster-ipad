//
//  fs_sync_content_hasher.swift
//  Fluster Desktop
//

import CryptoKit
import Foundation

/// Produces hashes comparable to the server's `content_hash`: lowercase hex
/// SHA-256 of the UTF-8 bytes. A note whose body was synced from a file hashes
/// identically to that file until either side changes.
nonisolated enum FsSyncContentHasher {
  static func hash(_ text: String) -> String {
    SHA256.hash(data: Data(text.utf8)).map { String(format: "%02x", $0) }.joined()
  }
}
