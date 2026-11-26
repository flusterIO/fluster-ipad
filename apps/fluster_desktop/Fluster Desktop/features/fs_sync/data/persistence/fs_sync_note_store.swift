//
//  fs_sync_note_store.swift
//  Fluster Desktop
//

import ConundrumSwift
import FlusterData
import Foundation
import SwiftData

nonisolated enum FsSyncApplyOutcome: Sendable, Equatable {
  case applied
  /// The change was saved but something non-fatal went wrong, e.g. parsing.
  case appliedWithWarning(String)
  case failed(String)
}

/// Where synced notes are written. The service depends on this protocol so the
/// persistence layer can change (e.g. a background `ModelActor`) without
/// touching sync logic, and so the service can be tested with an in-memory fake.
@MainActor
protocol FsSyncNoteStore: AnyObject {
  /// Notes that could match a file: those with an `fsPath` or a front matter `id`.
  func snapshot() throws -> [FsSyncNoteSnapshot]
  func createNote(from file: FsFileContent) async -> FsSyncApplyOutcome
  func updateNote(noteId: String, with file: FsFileContent) async -> FsSyncApplyOutcome
  func relinkNote(noteId: String, to relativePath: String) -> FsSyncApplyOutcome
  func detachNote(noteId: String) -> FsSyncApplyOutcome
  func deleteNote(noteId: String) -> FsSyncApplyOutcome
  func save() throws
}

/// Writes to SwiftData through the main context, matching how the rest of the
/// app edits notes. Parsing itself runs off the main thread inside
/// `NoteModel.preParse`.
final class SwiftDataFsSyncNoteStore: FsSyncNoteStore {
  private let modelContext: ModelContext
  private let uiParams: UiParams
  private var notesById: [String: NoteModel] = [:]

  init(modelContext: ModelContext, uiParams: UiParams) {
    self.modelContext = modelContext
    self.uiParams = uiParams
  }

  func snapshot() throws -> [FsSyncNoteSnapshot] {
    let notes = try modelContext.fetch(FetchDescriptor<NoteModel>())
    notesById = Dictionary(notes.map { ($0.id, $0) }, uniquingKeysWith: { first, _ in first })
    return notes.compactMap { note in
      let fsPath = note.frontMatter.fsPath.map(FsSyncPath.normalize).flatMap { $0.isEmpty ? nil : $0 }
      let userDefinedId = note.frontMatter.userDefinedId
      // A note with neither can never match a file, so skip hashing its body.
      guard fsPath != nil || userDefinedId != nil else { return nil }
      return FsSyncNoteSnapshot(
        noteId: note.id,
        fsPath: fsPath,
        userDefinedId: userDefinedId,
        contentHash: FsSyncContentHasher.hash(note.markdown.body),
        modifiedAt: note.utime
      )
    }
  }

  func createNote(from file: FsFileContent) async -> FsSyncApplyOutcome {
    let note = NoteModel.fromNoteBody(noteBody: file.content)
    note.frontMatter.fsPath = FsSyncPath.normalize(file.relativePath)
    note.ctime = file.createdAt ?? file.modifiedAt
    note.utime = file.modifiedAt
    modelContext.insert(note)
    notesById[note.id] = note
    return await parse(note)
  }

  func updateNote(noteId: String, with file: FsFileContent) async -> FsSyncApplyOutcome {
    guard let note = liveNote(noteId) else {
      return .failed("The matching note was deleted during the sync.")
    }
    note.markdown.body = file.content
    note.frontMatter.fsPath = FsSyncPath.normalize(file.relativePath)
    note.utime = file.modifiedAt
    note.updateTitleGroup()
    return await parse(note)
  }

  func relinkNote(noteId: String, to relativePath: String) -> FsSyncApplyOutcome {
    guard let note = liveNote(noteId) else {
      return .failed("The matching note was deleted during the sync.")
    }
    note.frontMatter.fsPath = FsSyncPath.normalize(relativePath)
    return .applied
  }

  func detachNote(noteId: String) -> FsSyncApplyOutcome {
    guard let note = liveNote(noteId) else { return .applied }
    note.frontMatter.fsPath = nil
    return .applied
  }

  func deleteNote(noteId: String) -> FsSyncApplyOutcome {
    guard let note = liveNote(noteId) else { return .applied }
    modelContext.delete(note)
    notesById.removeValue(forKey: noteId)
    return .applied
  }

  func save() throws {
    if modelContext.hasChanges {
      try modelContext.save()
    }
  }

  private func liveNote(_ noteId: String) -> NoteModel? {
    guard let note = notesById[noteId], !note.isDeleted else { return nil }
    return note
  }

  private func parse(_ note: NoteModel) async -> FsSyncApplyOutcome {
    do {
      try await note.preParse(modelContext: modelContext, uiParams: uiParams)
      return .applied
    } catch {
      // The raw body is saved either way; the note will be re-parsed when opened.
      return .appliedWithWarning("Synced, but couldn't be parsed: \(error.localizedDescription)")
    }
  }
}
