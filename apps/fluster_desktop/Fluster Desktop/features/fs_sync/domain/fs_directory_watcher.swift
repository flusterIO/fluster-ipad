//
//  fs_directory_watcher.swift
//  Fluster Desktop
//

import CoreServices
import Foundation

/// Calls `onChange` on the main thread when anything under `path` changes.
/// FSEvents coalesces bursts of changes (a git checkout, an editor's atomic
/// save) into one callback per `latency` window.
///
/// Call `stop()` before releasing the watcher.
final class FsDirectoryWatcher {
  private let path: String
  private let latency: CFTimeInterval
  private let onChange: () -> Void
  private var stream: FSEventStreamRef?

  init(path: String, latency: CFTimeInterval = 2, onChange: @escaping () -> Void) {
    self.path = path
    self.latency = latency
    self.onChange = onChange
  }

  @discardableResult
  func start() -> Bool {
    guard stream == nil else { return true }
    var context = FSEventStreamContext(
      version: 0, info: Unmanaged.passUnretained(self).toOpaque(),
      retain: nil, release: nil, copyDescription: nil)
    let flags = FSEventStreamCreateFlags(
      kFSEventStreamCreateFlagFileEvents | kFSEventStreamCreateFlagWatchRoot)
    guard
      let created = FSEventStreamCreate(
        kCFAllocatorDefault, fsDirectoryWatcherCallback, &context, [path] as CFArray,
        FSEventStreamEventId(kFSEventStreamEventIdSinceNow), latency, flags)
    else { return false }
    FSEventStreamSetDispatchQueue(created, DispatchQueue.main)
    guard FSEventStreamStart(created) else {
      FSEventStreamInvalidate(created)
      FSEventStreamRelease(created)
      return false
    }
    stream = created
    return true
  }

  func stop() {
    guard let stream else { return }
    FSEventStreamStop(stream)
    FSEventStreamInvalidate(stream)
    FSEventStreamRelease(stream)
    self.stream = nil
  }

  fileprivate func notifyChanged() {
    onChange()
  }
}

/// FSEvents needs a C function pointer, so this can't be a closure that captures state.
private nonisolated func fsDirectoryWatcherCallback(
  _ stream: ConstFSEventStreamRef,
  _ info: UnsafeMutableRawPointer?,
  _ eventCount: Int,
  _ eventPaths: UnsafeMutableRawPointer,
  _ eventFlags: UnsafePointer<FSEventStreamEventFlags>,
  _ eventIds: UnsafePointer<FSEventStreamEventId>
) {
  guard let info else { return }
  // The stream is scheduled on the main queue in `start()`.
  MainActor.assumeIsolated {
    Unmanaged<FsDirectoryWatcher>.fromOpaque(info).takeUnretainedValue().notifyChanged()
  }
}
