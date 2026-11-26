//
//  fs_note_file_kind.swift
//  Fluster Desktop
//

import Foundation

/// Mirrors `NoteFileKind` in `conundrum_server_rs/src/rest/fs_sync/note_file_kind.rs`.
nonisolated enum FsNoteFileKind: String, Codable, CaseIterable, Sendable, Hashable {
  case cdrm
  case markdown = "md"
  case mdx

  var fileExtension: String { rawValue }

  init?(relativePath: String) {
    guard let ext = relativePath.split(separator: ".").last, relativePath.contains(".") else {
      return nil
    }
    self.init(rawValue: ext.lowercased())
  }
}
