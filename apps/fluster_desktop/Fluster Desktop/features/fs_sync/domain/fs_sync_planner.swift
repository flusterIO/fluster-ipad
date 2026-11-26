//
//  fs_sync_planner.swift
//  Fluster Desktop
//

import Foundation

/// Decides what a sync should do by comparing the server's manifest with a
/// snapshot of the database. Pure: no I/O, no SwiftData, so it is cheap to test.
///
/// Matching, in order:
/// 1. A note whose `fsPath` equals the file's path.
/// 2. A note whose front matter `id` equals the file's `id`, provided that
///    note's own file is gone. This follows files that were moved or renamed.
/// Anything left over is a new note.
nonisolated struct FsSyncPlanner {
  let options: FsSyncOptions

  func plan(manifest: [FsManifestEntry], notes: [FsSyncNoteSnapshot]) -> FsSyncPlan {
    var plan = FsSyncPlan()

    // If several notes claim the same path or id, the most recently modified one wins.
    var notesByPath: [String: FsSyncNoteSnapshot] = [:]
    var notesByUserDefinedId: [String: FsSyncNoteSnapshot] = [:]
    for note in notes.sorted(by: { $0.modifiedAt > $1.modifiedAt }) {
      if let path = note.fsPath, notesByPath[path] == nil {
        notesByPath[path] = note
      }
      if let userDefinedId = note.userDefinedId, notesByUserDefinedId[userDefinedId] == nil {
        notesByUserDefinedId[userDefinedId] = note
      }
    }

    let manifestPaths = Set(manifest.map { FsSyncPath.normalize($0.relativePath) })
    var claimedNoteIds = Set<String>()

    // Pass 1: exact path matches. Done first so an id match can never take a
    // note whose own file still exists.
    var unmatched: [FsManifestEntry] = []
    for entry in manifest {
      let path = FsSyncPath.normalize(entry.relativePath)
      if let note = notesByPath[path], !claimedNoteIds.contains(note.noteId) {
        claimedNoteIds.insert(note.noteId)
        classify(entry: entry, note: note, relinked: false, into: &plan)
      } else {
        unmatched.append(entry)
      }
    }

    // Pass 2: front matter id matches.
    for entry in unmatched {
      if let userDefinedId = entry.frontMatter.userDefinedId,
        let note = notesByUserDefinedId[userDefinedId],
        !claimedNoteIds.contains(note.noteId),
        !(note.fsPath.map({ manifestPaths.contains($0) }) ?? false)
      {
        claimedNoteIds.insert(note.noteId)
        classify(entry: entry, note: note, relinked: true, into: &plan)
      } else {
        plan.creations.append(entry)
      }
    }

    // Synced notes whose file wasn't found.
    for note in notes where !claimedNoteIds.contains(note.noteId) {
      guard let path = note.fsPath, !manifestPaths.contains(path) else { continue }
      // A `.md` note isn't missing just because this sync excluded `.md` files.
      guard let kind = FsNoteFileKind(relativePath: path), options.fileKinds.contains(kind) else {
        continue
      }
      plan.missing.append(note)
    }

    return plan
  }

  private func classify(
    entry: FsManifestEntry, note: FsSyncNoteSnapshot, relinked: Bool, into plan: inout FsSyncPlan
  ) {
    if entry.contentHash == note.contentHash {
      if relinked {
        plan.relinks.append(.init(entry: entry, noteId: note.noteId))
      } else {
        plan.unchangedCount += 1
      }
      return
    }
    if shouldOverwrite(note: note, with: entry) {
      plan.updates.append(.init(entry: entry, noteId: note.noteId, relinked: relinked))
    } else {
      plan.conflicts.append(.init(entry: entry, noteId: note.noteId))
    }
  }

  private func shouldOverwrite(note: FsSyncNoteSnapshot, with entry: FsManifestEntry) -> Bool {
    switch options.conflictPolicy {
      case .preferFileSystem:
        return true
      case .preferDatabase:
        return false
      case .preferNewest:
        // Synced notes take their file's modification date, so a later edit in
        // Fluster makes the note newer and an edit on disk makes the file newer.
        return entry.modifiedAt > note.modifiedAt
    }
  }
}
