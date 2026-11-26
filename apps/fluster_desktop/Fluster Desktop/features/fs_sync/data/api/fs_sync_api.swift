//
//  fs_sync_api.swift
//  Fluster Desktop
//

import Foundation

/// The file system sync calls the app makes to the Conundrum server.
/// The service depends on this protocol, not on HTTP, so it can be tested with a fake.
nonisolated protocol FsSyncAPIProtocol: Sendable {
  func checkHealth() async throws -> ServerHealthResponse
  func fetchManifest(_ request: FsManifestRequest) async throws -> FsManifestResponse
  func fetchContents(_ request: FsContentsRequest) async throws -> FsContentsResponse
}

nonisolated struct FsSyncAPI: FsSyncAPIProtocol {
  let client: any ConundrumServerClientProtocol

  func checkHealth() async throws -> ServerHealthResponse {
    try await client.send(ServerHealthEndpoint())
  }

  func fetchManifest(_ request: FsManifestRequest) async throws -> FsManifestResponse {
    try await client.send(FsSyncEndpoints.Manifest(request))
  }

  func fetchContents(_ request: FsContentsRequest) async throws -> FsContentsResponse {
    try await client.send(FsSyncEndpoints.Contents(request))
  }
}
