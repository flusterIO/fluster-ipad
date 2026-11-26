//
//  fs_sync_contents_dto.swift
//  Fluster Desktop
//

import Foundation

// Wire types for `POST /api/v1/fs_sync/contents`.

nonisolated struct FsContentsRequest: Encodable, Sendable, Equatable {
  let root: String
  let relativePaths: [String]
}

nonisolated struct FsContentsResponse: Decodable, Sendable, Equatable {
  let protocolVersion: Int
  let files: [FsFileContent]
  let failures: [FsFileIssue]
}

nonisolated struct FsFileContent: Decodable, Sendable, Equatable {
  let relativePath: String
  let content: String
  let contentHash: String
  let modifiedAtMs: Int64
  let createdAtMs: Int64?

  var modifiedAt: Date { Date(millisecondsSince1970: modifiedAtMs) }
  var createdAt: Date? { createdAtMs.map { Date(millisecondsSince1970: $0) } }
}
