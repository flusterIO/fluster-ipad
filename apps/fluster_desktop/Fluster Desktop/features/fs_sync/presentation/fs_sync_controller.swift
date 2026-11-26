//
//  fs_sync_controller.swift
//  Fluster Desktop
//

import ConundrumSwift
import FlusterData
import Foundation
import Observation
import SwiftData

/// App-wide owner of file system sync. Injected into the environment by
/// `Fluster_DesktopApp`.
///
/// - `syncNow()` is the one call to attach to any button or menu item.
/// - `activate()` (called once at launch) starts the file watcher when it's enabled.
@Observable
final class FsSyncController {
  enum Status: Equatable {
    case idle
    case running(FsSyncProgress)
    case finished(FsSyncReport)
    case failed(String)
  }

  private(set) var status: Status = .idle
  private(set) var lastReport: FsSyncReport?
  private(set) var isWatching = false

  private let service: FsSyncService
  @ObservationIgnored private let modelContainer: ModelContainer
  @ObservationIgnored private var activeTask: Task<Void, Never>?
  @ObservationIgnored private var watcher: FsDirectoryWatcher?
  @ObservationIgnored private var watchedDirectory: URL?
  @ObservationIgnored private var hasPendingSync = false

  init(api: any FsSyncAPIProtocol, modelContainer: ModelContainer) {
    self.service = FsSyncService(api: api)
    self.modelContainer = modelContainer
  }

  convenience init(
    modelContainer: ModelContainer,
    client: any ConundrumServerClientProtocol = ConundrumServerClient()
  ) {
    self.init(api: FsSyncAPI(client: client), modelContainer: modelContainer)
  }

  var isRunning: Bool {
    if case .running = status { return true }
    return false
  }

  // MARK: - Syncing

  /// Syncs the notes directory using the settings saved in Settings.
  /// Calls made while a sync is running queue a single follow-up sync, so
  /// rapid file changes never get lost or pile up.
  func syncNow() {
    if isRunning {
      hasPendingSync = true
      return
    }
    let defaults = UserDefaults.standard
    startSync(
      notesDirectory: defaults.string(forKey: AppStorageKeys.notesDirectory.rawValue) ?? "",
      options: Self.storedOptions(defaults),
      modelContext: modelContainer.mainContext,
      uiParams: Self.storedUiParams(defaults)
    )
  }

  func cancel() {
    hasPendingSync = false
    activeTask?.cancel()
  }

  /// Lower-level entry point for callers that want explicit options.
  func startSync(
    notesDirectory: String, options: FsSyncOptions, modelContext: ModelContext, uiParams: UiParams
  ) {
    guard !isRunning else { return }
    let rootPath: String
    do {
      rootPath = try NotesDirectoryResolver.resolve(notesDirectory)
    } catch {
      status = .failed(error.localizedDescription)
      return
    }

    let store = SwiftDataFsSyncNoteStore(modelContext: modelContext, uiParams: uiParams)
    status = .running(.checkingServer)
    activeTask = Task { [weak self] in
      guard let self else { return }
      do {
        let report = try await self.service.sync(rootPath: rootPath, options: options, store: store) {
          [weak self] progress in
          self?.status = .running(progress)
        }
        self.lastReport = report
        self.status = .finished(report)
        UserDefaults.standard.set(
          report.finishedAt.timeIntervalSince1970, forKey: FsSyncStorageKeys.lastSyncedAt.rawValue)
      } catch is CancellationError {
        self.status = .idle
      } catch {
        self.status = .failed(error.localizedDescription)
      }
      self.activeTask = nil
      if self.hasPendingSync {
        self.hasPendingSync = false
        self.syncNow()
      }
    }
  }

  // MARK: - Watching

  static var isWatchingEnabled: Bool {
    UserDefaults.standard.bool(forKey: FsSyncStorageKeys.watchForChanges.rawValue)
  }

  /// Call once at launch. If watching is enabled, starts the watcher and runs a catch-up sync.
  func activate() {
    guard Self.isWatchingEnabled else { return }
    startWatching()
    syncNow()
  }

  /// (Re)starts the watcher on the bookmarked notes directory. Call after the
  /// directory or the watch setting changes.
  func startWatching() {
    stopWatching()
    guard let bookmark = NotesDirectoryBookmark.resolve() else {
      status = .failed(
        "To sync when files change, choose your notes directory again in Settings so Fluster can watch it.")
      return
    }
    let url = bookmark.url
    if url.startAccessingSecurityScopedResource() {
      watchedDirectory = url
    }
    if bookmark.isStale {
      try? NotesDirectoryBookmark.save(url)
    }
    let watcher = FsDirectoryWatcher(path: url.path(percentEncoded: false)) { [weak self] in
      self?.syncNow()
    }
    isWatching = watcher.start()
    self.watcher = isWatching ? watcher : nil
    if !isWatching {
      status = .failed("Couldn't watch \(url.path(percentEncoded: false)) for changes.")
    }
  }

  func stopWatching() {
    watcher?.stop()
    watcher = nil
    watchedDirectory?.stopAccessingSecurityScopedResource()
    watchedDirectory = nil
    isWatching = false
  }

  // MARK: - Stored settings

  private static func storedOptions(_ defaults: UserDefaults) -> FsSyncOptions {
    var options = FsSyncOptions()
    options.respectGitignore =
      defaults.object(forKey: AppStorageKeys.respectGitIgnore.rawValue) as? Bool ?? true
    if let raw = defaults.string(forKey: FsSyncStorageKeys.conflictPolicy.rawValue),
      let policy = FsSyncConflictPolicy(rawValue: raw)
    {
      options.conflictPolicy = policy
    }
    if let raw = defaults.string(forKey: FsSyncStorageKeys.missingFilePolicy.rawValue),
      let policy = FsSyncMissingFilePolicy(rawValue: raw)
    {
      options.missingFilePolicy = policy
    }
    return options
  }

  private static func storedUiParams(_ defaults: UserDefaults) -> UiParams {
    let theme =
      defaults.string(forKey: AppStorageKeys.colorScheme.rawValue).flatMap(AppTheme.init(rawValue:))
      ?? .dark
    return UIParamsProvider().toParams(colorScheme: theme.colorScheme)
  }
}
