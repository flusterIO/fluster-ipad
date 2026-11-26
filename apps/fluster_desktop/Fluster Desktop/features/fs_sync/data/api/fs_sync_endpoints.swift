//
//  fs_sync_endpoints.swift
//  Fluster Desktop
//

import Foundation

/// Routes under `/api/v1/fs_sync`. Mirrors `conundrum_server_rs/src/rest/fs_sync/router.rs`.
nonisolated enum FsSyncEndpoints {
  /// The protocol version this build of the app understands.
  static let supportedProtocolVersion = 1
  /// Mirrors `MAX_CONTENT_PATHS_PER_REQUEST` on the server.
  static let maxContentPathsPerRequest = 500

  nonisolated struct Manifest: ConundrumServerEndpoint {
    typealias Response = FsManifestResponse
    let body: FsManifestRequest?
    var method: ConundrumHTTPMethod { .post }
    var path: String { "fs_sync/manifest" }
    /// Hashing a large notes directory can take a while on a cold disk.
    var timeout: TimeInterval? { 300 }

    init(_ request: FsManifestRequest) { self.body = request }
  }

  nonisolated struct Contents: ConundrumServerEndpoint {
    typealias Response = FsContentsResponse
    let body: FsContentsRequest?
    var method: ConundrumHTTPMethod { .post }
    var path: String { "fs_sync/contents" }
    var timeout: TimeInterval? { 120 }

    init(_ request: FsContentsRequest) { self.body = request }
  }
}
