//
//  fs_sync_planner_tests.swift
//  Fluster DesktopTests
//

import Foundation
import Testing

@testable import Fluster

struct FsSyncPlannerTests {
  private let earlier = Date(timeIntervalSince1970: 1_000)
  private let later = Date(timeIntervalSince1970: 2_000)

  private func entry(
    _ path: String, hash: String, modifiedAt: Date, id: String? = nil
  ) -> FsManifestEntry {
    FsManifestEntry(
      relativePath: path,
      fileKind: FsNoteFileKind(relativePath: path) ?? .cdrm,
      sizeBytes: 10,
      modifiedAtMs: Int64(modifiedAt.timeIntervalSince1970 * 1000),
      createdAtMs: nil,
      contentHash: hash,
      frontMatter: FsManifestFrontMatter(userDefinedId: id, title: nil)
    )
  }

  private func note(
    _ noteId: String, path: String?, hash: String, modifiedAt: Date, id: String? = nil
  ) -> FsSyncNoteSnapshot {
    FsSyncNoteSnapshot(
      noteId: noteId, fsPath: path, userDefinedId: id, contentHash: hash, modifiedAt: modifiedAt)
  }

  private func planner(_ configure: (inout FsSyncOptions) -> Void = { _ in }) -> FsSyncPlanner {
    var options = FsSyncOptions()
    configure(&options)
    return FsSyncPlanner(options: options)
  }

  @Test func createsNotesForUnmatchedFiles() {
    let plan = planner().plan(manifest: [entry("a.cdrm", hash: "1", modifiedAt: later)], notes: [])
    #expect(plan.creations.map(\.relativePath) == ["a.cdrm"])
    #expect(plan.pathsToFetch == ["a.cdrm"])
  }

  @Test func leavesIdenticalNotesAlone() {
    let plan = planner().plan(
      manifest: [entry("a.cdrm", hash: "1", modifiedAt: later)],
      notes: [note("n1", path: "a.cdrm", hash: "1", modifiedAt: earlier)])
    #expect(plan.unchangedCount == 1)
    #expect(plan.pathsToFetch.isEmpty)
  }

  @Test func newestWinsByDefault() {
    let fileNewer = planner().plan(
      manifest: [entry("a.cdrm", hash: "2", modifiedAt: later)],
      notes: [note("n1", path: "a.cdrm", hash: "1", modifiedAt: earlier)])
    #expect(fileNewer.updates.map(\.noteId) == ["n1"])

    let noteNewer = planner().plan(
      manifest: [entry("a.cdrm", hash: "2", modifiedAt: earlier)],
      notes: [note("n1", path: "a.cdrm", hash: "1", modifiedAt: later)])
    #expect(noteNewer.updates.isEmpty)
    #expect(noteNewer.conflicts.map(\.noteId) == ["n1"])
  }

  @Test func conflictPolicyOverridesTimestamps() {
    let manifest = [entry("a.cdrm", hash: "2", modifiedAt: earlier)]
    let notes = [note("n1", path: "a.cdrm", hash: "1", modifiedAt: later)]
    let fileSystemWins = planner { $0.conflictPolicy = .preferFileSystem }.plan(
      manifest: manifest, notes: notes)
    #expect(fileSystemWins.updates.count == 1)

    let databaseWins = planner { $0.conflictPolicy = .preferDatabase }.plan(
      manifest: [entry("a.cdrm", hash: "2", modifiedAt: later)],
      notes: [note("n1", path: "a.cdrm", hash: "1", modifiedAt: earlier)])
    #expect(databaseWins.updates.isEmpty)
    #expect(databaseWins.conflicts.count == 1)
  }

  @Test func followsMovedFilesByFrontMatterId() {
    let unchangedMove = planner().plan(
      manifest: [entry("new/a.cdrm", hash: "1", modifiedAt: later, id: "intro")],
      notes: [note("n1", path: "old/a.cdrm", hash: "1", modifiedAt: earlier, id: "intro")])
    #expect(unchangedMove.relinks.map(\.noteId) == ["n1"])
    #expect(unchangedMove.creations.isEmpty)
    #expect(unchangedMove.missing.isEmpty)

    let editedMove = planner().plan(
      manifest: [entry("new/a.cdrm", hash: "2", modifiedAt: later, id: "intro")],
      notes: [note("n1", path: "old/a.cdrm", hash: "1", modifiedAt: earlier, id: "intro")])
    #expect(editedMove.updates.first?.relinked == true)
  }

  @Test func idMatchNeverStealsANoteWhoseFileStillExists() {
    let plan = planner().plan(
      manifest: [
        entry("a.cdrm", hash: "1", modifiedAt: later, id: "dup"),
        entry("copy.cdrm", hash: "1", modifiedAt: later, id: "dup"),
      ],
      notes: [note("n1", path: "a.cdrm", hash: "1", modifiedAt: earlier, id: "dup")])
    #expect(plan.unchangedCount == 1)
    #expect(plan.creations.map(\.relativePath) == ["copy.cdrm"])
  }

  @Test func reportsMissingFilesOnlyForSyncedKinds() {
    let notes = [
      note("n1", path: "gone.cdrm", hash: "1", modifiedAt: earlier),
      note("n2", path: "gone.md", hash: "1", modifiedAt: earlier),
      note("n3", path: nil, hash: "1", modifiedAt: earlier, id: "app-only"),
    ]
    let plan = planner { $0.fileKinds = [.cdrm] }.plan(manifest: [], notes: notes)
    #expect(plan.missing.map(\.noteId) == ["n1"])
  }

  @Test func hashesMatchTheServer() {
    // Same SHA-256 reference vector as `note_file.rs`.
    #expect(
      FsSyncContentHasher.hash("abc")
        == "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad")
  }

  @Test func normalizesPathsAndNotesDirectories() throws {
    #expect(FsSyncPath.normalize("./a//b/./c.cdrm") == "a/b/c.cdrm")
    #expect(try NotesDirectoryResolver.resolve("/Users/me/My%20Notes/") == "/Users/me/My Notes/")
    #expect(try NotesDirectoryResolver.resolve("file:///Users/me/My%20Notes") == "/Users/me/My Notes")
    #expect(throws: FsSyncError.notesDirectoryNotSet) { try NotesDirectoryResolver.resolve("  ") }
  }
}
