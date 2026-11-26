//
//  conundrum_server_error.swift
//  Fluster Desktop
//

import Foundation

/// Every error thrown by `ConundrumServerClient`.
nonisolated enum ConundrumServerError: LocalizedError, Sendable, Equatable {
  /// Nothing is listening at the configured address. Usually the server isn't running.
  case serverUnreachable(url: String)
  case timedOut
  case transport(message: String)
  case invalidResponse
  /// The server answered with a non-2xx status. `code` is the machine readable
  /// code from the server's error envelope, when it sent one.
  case http(status: Int, code: String?, message: String)
  case encoding(message: String)
  case decoding(message: String)

  var errorDescription: String? {
    switch self {
      case .serverUnreachable(let url):
        return "The Conundrum server isn't reachable at \(url). Make sure `cdrm_server` is running."
      case .timedOut:
        return "The Conundrum server took too long to respond."
      case .transport(let message):
        return "Couldn't talk to the Conundrum server: \(message)"
      case .invalidResponse:
        return "The Conundrum server sent a response that isn't HTTP."
      case .http(let status, _, let message):
        return "The Conundrum server returned an error (\(status)): \(message)"
      case .encoding(let message):
        return "Couldn't encode the request: \(message)"
      case .decoding(let message):
        return "Couldn't read the server's response. The app and server may be out of date with each other. \(message)"
    }
  }

  static func fromURLError(_ error: URLError, url: URL?) -> ConundrumServerError {
    switch error.code {
      case .cannotConnectToHost, .cannotFindHost, .networkConnectionLost, .notConnectedToInternet,
        .dnsLookupFailed:
        return .serverUnreachable(url: url?.absoluteString ?? "unknown")
      case .timedOut:
        return .timedOut
      default:
        return .transport(message: error.localizedDescription)
    }
  }
}

/// Mirrors the server's `{ "error": { "code": ..., "message": ... } }` envelope.
nonisolated struct ConundrumServerErrorEnvelope: Decodable, Sendable {
  nonisolated struct Detail: Decodable, Sendable {
    let code: String
    let message: String
  }
  let error: Detail
}
