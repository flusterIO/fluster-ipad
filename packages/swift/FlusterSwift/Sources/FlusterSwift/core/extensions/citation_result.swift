import FlusterRust
import Foundation

extension CitationResult: Codable {
  public enum CodingKeys: String, CodingKey {
    case body
    case citationKey
  }

  public init(from decoder: Decoder) throws {
    let container = try decoder.container(keyedBy: CodingKeys.self)
    let body = try container.decode(String.self, forKey: .body)
    let citationKey = try container.decode(String.self, forKey: .citationKey)

    self.init(citationKey: citationKey, body: body)
  }

  public func encode(to encoder: any Encoder) throws {
    var container = encoder.container(keyedBy: CodingKeys.self)

    try container.encode(body, forKey: .body)
    try container.encode(citationKey, forKey: .citationKey)
  }
}
