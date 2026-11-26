//
//  fs_sync_note_snapshot.swift
//  Fluster Desktop
//

import Foundation

/// A value-type view of a `NoteModel`, holding only what the planner needs.
/// Keeps planning free of SwiftData so it is pure and testable.
nonisolated struct FsSyncNoteSnapshot: Sendable, Equatable {
  /// `NoteModel.id`.
  let noteId: String
  /// `FrontMatter.fsPath`, normalized.
  let fsPath: String?
  let userDefinedId: String?
  /// SHA-256 of the note's markdown body, comparable to `FsManifestEntry.contentHash`.
  let contentHash: String
  let modifiedAt: Date
}
