//
//  fs_sync_service.swift
//  Fluster Desktop
//

import Foundation

/// Runs one file system → database sync:
///
///     health check → manifest → plan → fetch changed contents in batches → apply → missing files
///
/// The server only reads files; every database decision is made here. Each
/// batch is saved as it completes, so a cancelled or failed sync keeps the
/// work it finished.
final class FsSyncService {
  private let api: any FsSyncAPIProtocol

  init(api: any FsSyncAPIProtocol) {
    self.api = api
  }

  func sync(
    rootPath: String,
    options: FsSyncOptions,
    store: any FsSyncNoteStore,
    progress: (FsSyncProgress) -> Void
  ) async throws -> FsSyncReport {
    var report = FsSyncReport(rootPath: rootPath, startedAt: .now)
    let supportedVersion = FsSyncEndpoints.supportedProtocolVersion

    progress(.checkingServer)
    let health = try await api.checkHealth()
    guard health.fsSyncProtocolVersion == supportedVersion else {
      throw FsSyncError.incompatibleServer(
        serverVersion: health.fsSyncProtocolVersion, appVersion: supportedVersion)
    }

    progress(.scanning)
    let manifest = try await api.fetchManifest(
      FsManifestRequest(
        root: rootPath,
        respectGitignore: options.respectGitignore,
        includeHidden: options.includeHidden,
        fileKinds: FsNoteFileKind.allCases.filter { options.fileKinds.contains($0) }
      ))
    guard manifest.protocolVersion == supportedVersion else {
      throw FsSyncError.incompatibleServer(
        serverVersion: manifest.protocolVersion, appVersion: supportedVersion)
    }
    report.rootPath = manifest.root
    report.scannedFileCount = manifest.entries.count
    report.failures += manifest.skipped.map(Self.issue(from:))

    progress(.planning)
    let snapshot = try store.snapshot()
    let plan = FsSyncPlanner(options: options).plan(manifest: manifest.entries, notes: snapshot)
    report.unchangedCount = plan.unchangedCount
    report.conflicts = plan.conflicts.map {
      FsSyncIssue(
        relativePath: $0.entry.relativePath,
        message: "Changed in Fluster since it was last synced; kept the Fluster version.")
    }

    for relink in plan.relinks {
      record(
        store.relinkNote(noteId: relink.noteId, to: relink.entry.relativePath),
        path: relink.entry.relativePath, counter: \.relinkedCount, in: &report)
    }

    do {
      try await applyContentChanges(
        plan: plan, root: manifest.root, options: options, store: store, report: &report,
        progress: progress)
    } catch is CancellationError {
      try? store.save()
      report.wasCancelled = true
      report.finishedAt = .now
      return report
    }

    progress(.finishing)
    applyMissingFilePolicy(
      plan: plan, manifest: manifest, options: options, store: store, report: &report)
    try store.save()

    report.finishedAt = .now
    return report
  }

  // MARK: - Steps

  private func applyContentChanges(
    plan: FsSyncPlan,
    root: String,
    options: FsSyncOptions,
    store: any FsSyncNoteStore,
    report: inout FsSyncReport,
    progress: (FsSyncProgress) -> Void
  ) async throws {
    let updatesByPath = Dictionary(
      plan.updates.map { ($0.entry.relativePath, $0) }, uniquingKeysWith: { first, _ in first })
    let creationPaths = Set(plan.creations.map(\.relativePath))
    let paths = plan.pathsToFetch
    var completed = 0
    progress(.applying(completed: 0, total: paths.count))

    for batch in paths.fsSyncChunked(into: options.effectiveBatchSize) {
      try Task.checkCancellation()
      let response = try await api.fetchContents(
        FsContentsRequest(root: root, relativePaths: batch))
      report.failures += response.failures.map(Self.issue(from:))

      for file in response.files {
        if let update = updatesByPath[file.relativePath] {
          let outcome = await store.updateNote(noteId: update.noteId, with: file)
          let counter: WritableKeyPath<FsSyncReport, Int> =
            update.relinked ? \.relinkedCount : \.updatedCount
          record(outcome, path: file.relativePath, counter: counter, in: &report)
        } else if creationPaths.contains(file.relativePath) {
          let outcome = await store.createNote(from: file)
          record(outcome, path: file.relativePath, counter: \.createdCount, in: &report)
        }
      }

      try store.save()
      completed += batch.count
      progress(.applying(completed: completed, total: paths.count))
    }
  }

  private func applyMissingFilePolicy(
    plan: FsSyncPlan,
    manifest: FsManifestResponse,
    options: FsSyncOptions,
    store: any FsSyncNoteStore,
    report: inout FsSyncReport
  ) {
    guard !plan.missing.isEmpty else { return }

    // An empty directory almost always means the wrong folder or an unmounted
    // drive, not that every note was deleted. Never act destructively on that.
    if manifest.entries.isEmpty && options.missingFilePolicy != .keepNote {
      report.missingKeptCount += plan.missing.count
      report.warnings.append(
        FsSyncIssue(
          relativePath: manifest.root,
          message:
            "No note files were found, so \(plan.missing.count) synced notes were left untouched as a precaution."
        ))
      return
    }

    for note in plan.missing {
      let path = note.fsPath ?? note.noteId
      switch options.missingFilePolicy {
        case .keepNote:
          report.missingKeptCount += 1
        case .detachNote:
          record(store.detachNote(noteId: note.noteId), path: path, counter: \.detachedCount, in: &report)
        case .deleteNote:
          record(store.deleteNote(noteId: note.noteId), path: path, counter: \.deletedCount, in: &report)
      }
    }
  }

  // MARK: - Helpers

  private func record(
    _ outcome: FsSyncApplyOutcome, path: String, counter: WritableKeyPath<FsSyncReport, Int>,
    in report: inout FsSyncReport
  ) {
    switch outcome {
      case .applied:
        report[keyPath: counter] += 1
      case .appliedWithWarning(let message):
        report[keyPath: counter] += 1
        report.warnings.append(FsSyncIssue(relativePath: path, message: message))
      case .failed(let message):
        report.failures.append(FsSyncIssue(relativePath: path, message: message))
    }
  }

  private static func issue(from fileIssue: FsFileIssue) -> FsSyncIssue {
    FsSyncIssue(relativePath: fileIssue.relativePath, message: fileIssue.reason.userFacingDescription)
  }
}

extension Array {
  fileprivate nonisolated func fsSyncChunked(into size: Int) -> [[Element]] {
    guard size > 0, !isEmpty else { return isEmpty ? [] : [self] }
    return stride(from: 0, to: count, by: size).map {
      Array(self[$0..<Swift.min($0 + size, count)])
    }
  }
}
