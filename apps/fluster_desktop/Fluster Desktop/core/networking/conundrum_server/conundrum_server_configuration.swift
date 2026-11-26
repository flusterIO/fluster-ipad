//
//  conundrum_server_configuration.swift
//  Fluster Desktop
//

import Foundation

/// Where to reach the locally running `cdrm_server`.
///
/// The server binds to `127.0.0.1` only. An IP literal is used rather than
/// `localhost` so App Transport Security never interferes with plain HTTP.
nonisolated struct ConundrumServerConfiguration: Sendable, Equatable {
  /// Mirrors `DEFAULT_CDRM_SERVER_PORT` in the `conundrum` crate.
  static let defaultPort: Int = 3005
  /// Mirrors `REST_API_PREFIX` in `conundrum_server_rs/src/rest/rest_router.rs`.
  static let restApiPrefix: String = "/api/v1"

  var host: String = "127.0.0.1"
  var port: Int = ConundrumServerConfiguration.defaultPort
  var apiPrefix: String = ConundrumServerConfiguration.restApiPrefix
  /// Used for any endpoint that doesn't specify its own timeout.
  var defaultTimeout: TimeInterval = 30

  var baseURL: URL {
    var components = URLComponents()
    components.scheme = "http"
    components.host = host
    components.port = port
    components.path = apiPrefix
    // Every component above is a constant or a validated integer, so this cannot fail.
    return components.url!
  }

  /// Reads the port override from `UserDefaults`, falling back to the default port.
  static func current(defaults: UserDefaults = .standard) -> ConundrumServerConfiguration {
    var configuration = ConundrumServerConfiguration()
    let storedPort = defaults.integer(forKey: ConundrumServerStorageKeys.serverPort.rawValue)
    if (1...65535).contains(storedPort) {
      configuration.port = storedPort
    }
    return configuration
  }
}

/// `UserDefaults` keys owned by the server integration.
nonisolated enum ConundrumServerStorageKeys: String {
  case serverPort = "conundrumServerPort"
}
