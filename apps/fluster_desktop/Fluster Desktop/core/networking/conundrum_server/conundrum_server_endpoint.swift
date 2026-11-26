//
//  conundrum_server_endpoint.swift
//  Fluster Desktop
//

import Foundation

nonisolated enum ConundrumHTTPMethod: String, Sendable {
  case get = "GET"
  case post = "POST"
  case put = "PUT"
  case patch = "PATCH"
  case delete = "DELETE"
}

/// A single route on the Conundrum server's REST API.
///
/// To call a new server feature, declare a type conforming to this protocol
/// next to that feature's code and pass it to `ConundrumServerClient.send(_:)`.
/// Transport, JSON coding (snake_case on the wire) and error mapping are
/// handled by the client.
nonisolated protocol ConundrumServerEndpoint: Sendable {
  associatedtype RequestBody: Encodable & Sendable = ConundrumEmptyBody
  associatedtype Response: Decodable & Sendable

  var method: ConundrumHTTPMethod { get }
  /// Relative to the configured API prefix, without a leading slash, e.g. `fs_sync/manifest`.
  var path: String { get }
  var body: RequestBody? { get }
  /// Overrides `ConundrumServerConfiguration.defaultTimeout` for slow routes.
  var timeout: TimeInterval? { get }
}

extension ConundrumServerEndpoint {
  var timeout: TimeInterval? { nil }
}

extension ConundrumServerEndpoint where RequestBody == ConundrumEmptyBody {
  var body: ConundrumEmptyBody? { nil }
}

nonisolated struct ConundrumEmptyBody: Codable, Sendable {}
