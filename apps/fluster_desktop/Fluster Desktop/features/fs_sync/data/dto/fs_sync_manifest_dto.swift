//
//  fs_sync_manifest_dto.swift
//  Fluster Desktop
//

import Foundation

// Wire types for `POST /api/v1/fs_sync/manifest`.
// Mirrors `conundrum_server_rs/src/rest/fs_sync/models.rs`.

nonisolated struct FsManifestRequest: Encodable, Sendable, Equatable {
  /// Absolute path of the notes directory.
  let root: String
  let respectGitignore: Bool
  let includeHidden: Bool
  let fileKinds: [FsNoteFileKind]
}

nonisolated struct FsManifestResponse: Decodable, Sendable, Equatable {
  let protocolVersion: Int
  /// The canonical root that every `relativePath` is relative to.
  let root: String
  let generatedAtMs: Int64
  let entries: [FsManifestEntry]
  let skipped: [FsFileIssue]
}

nonisolated struct FsManifestEntry: Decodable, Sendable, Equatable {
  /// `/`-separated and relative to the root. This is the file's identity, and is
  /// what gets stored in `FrontMatter.fsPath`.
  let relativePath: String
  let fileKind: FsNoteFileKind
  let sizeBytes: UInt64
  let modifiedAtMs: Int64
  let createdAtMs: Int64?
  /// Lowercase hex SHA-256 of the file's bytes.
  let contentHash: String
  let frontMatter: FsManifestFrontMatter

  var modifiedAt: Date { Date(millisecondsSince1970: modifiedAtMs) }
}

nonisolated struct FsManifestFrontMatter: Decodable, Sendable, Equatable {
  let userDefinedId: String?
  let title: String?
}

nonisolated struct FsFileIssue: Decodable, Sendable, Equatable {
  let relativePath: String
  let reason: FsFileIssueReason
}

/// Mirrors `FileIssueReason`. Unknown values decode to `.other` so a newer
/// server never breaks an older app.
nonisolated enum FsFileIssueReason: Decodable, Sendable, Equatable {
  case tooLarge, notUtf8, unreadable, notFound, notAFile, outsideRoot, unsupportedFileKind
  case other(String)

  init(from decoder: Decoder) throws {
    let value = try decoder.singleValueContainer().decode(String.self)
    switch value {
      case "too_large": self = .tooLarge
      case "not_utf8": self = .notUtf8
      case "unreadable": self = .unreadable
      case "not_found": self = .notFound
      case "not_a_file": self = .notAFile
      case "outside_root": self = .outsideRoot
      case "unsupported_file_kind": self = .unsupportedFileKind
      default: self = .other(value)
    }
  }

  var userFacingDescription: String {
    switch self {
      case .tooLarge: return "File is too large to sync."
      case .notUtf8: return "File isn't valid UTF-8 text."
      case .unreadable: return "File couldn't be read."
      case .notFound: return "File no longer exists."
      case .notAFile: return "Path isn't a regular file."
      case .outsideRoot: return "Path is outside the notes directory."
      case .unsupportedFileKind: return "File type isn't supported."
      case .other(let value): return "Skipped (\(value))."
    }
  }
}

extension Date {
  nonisolated init(millisecondsSince1970 ms: Int64) {
    self.init(timeIntervalSince1970: TimeInterval(ms) / 1000)
  }
}
