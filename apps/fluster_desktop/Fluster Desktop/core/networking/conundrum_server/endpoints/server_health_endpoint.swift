//
//  server_health_endpoint.swift
//  Fluster Desktop
//

import Foundation

/// `GET /api/v1/health`. Cheap; use it to check the server is running before longer work.
nonisolated struct ServerHealthEndpoint: ConundrumServerEndpoint {
  typealias Response = ServerHealthResponse
  var method: ConundrumHTTPMethod { .get }
  var path: String { "health" }
}

nonisolated struct ServerHealthResponse: Decodable, Sendable, Equatable {
  let status: String
  let serverVersion: String
  let fsSyncProtocolVersion: Int
}
