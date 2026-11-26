//
//  conundrum_server_client.swift
//  Fluster Desktop
//

import Foundation

/// Abstracts transport so features can be tested against a fake server.
nonisolated protocol ConundrumServerClientProtocol: Sendable {
  var configuration: ConundrumServerConfiguration { get }
  /// Throws `ConundrumServerError` (or `CancellationError` if the task is cancelled).
  func send<E: ConundrumServerEndpoint>(_ endpoint: E) async throws -> E.Response
}

/// The HTTP client for `cdrm_server`'s REST API. Stateless and safe to share.
nonisolated final class ConundrumServerClient: ConundrumServerClientProtocol {
  let configuration: ConundrumServerConfiguration
  private let session: URLSession

  init(configuration: ConundrumServerConfiguration = .current(), session: URLSession = .shared) {
    self.configuration = configuration
    self.session = session
  }

  func send<E: ConundrumServerEndpoint>(_ endpoint: E) async throws -> E.Response {
    let request = try makeRequest(for: endpoint)

    let data: Data
    let response: URLResponse
    do {
      (data, response) = try await session.data(for: request)
    } catch let error as URLError where error.code == .cancelled {
      throw CancellationError()
    } catch let error as URLError {
      throw ConundrumServerError.fromURLError(error, url: request.url)
    } catch is CancellationError {
      throw CancellationError()
    } catch {
      throw ConundrumServerError.transport(message: error.localizedDescription)
    }

    guard let httpResponse = response as? HTTPURLResponse else {
      throw ConundrumServerError.invalidResponse
    }
    guard (200..<300).contains(httpResponse.statusCode) else {
      throw Self.httpError(status: httpResponse.statusCode, data: data)
    }

    do {
      return try ConundrumServerCoding.makeDecoder().decode(E.Response.self, from: data)
    } catch {
      throw ConundrumServerError.decoding(message: String(describing: error))
    }
  }

  private func makeRequest<E: ConundrumServerEndpoint>(for endpoint: E) throws -> URLRequest {
    let url = configuration.baseURL.appending(path: endpoint.path)
    var request = URLRequest(url: url)
    request.httpMethod = endpoint.method.rawValue
    request.timeoutInterval = endpoint.timeout ?? configuration.defaultTimeout
    request.setValue("application/json", forHTTPHeaderField: "Accept")
    if let body = endpoint.body {
      do {
        request.httpBody = try ConundrumServerCoding.makeEncoder().encode(body)
      } catch {
        throw ConundrumServerError.encoding(message: String(describing: error))
      }
      request.setValue("application/json", forHTTPHeaderField: "Content-Type")
    }
    return request
  }

  private static func httpError(status: Int, data: Data) -> ConundrumServerError {
    if let envelope = try? ConundrumServerCoding.makeDecoder().decode(
      ConundrumServerErrorEnvelope.self, from: data)
    {
      return .http(status: status, code: envelope.error.code, message: envelope.error.message)
    }
    // Framework-level rejections (e.g. malformed JSON) come back as plain text.
    let text = String(data: data, encoding: .utf8)?.trimmingCharacters(in: .whitespacesAndNewlines)
    return .http(
      status: status, code: nil,
      message: (text?.isEmpty == false ? text! : HTTPURLResponse.localizedString(forStatusCode: status)))
  }
}
