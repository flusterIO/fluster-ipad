import FlusterRust
import Foundation

extension CitationResult: Codable {
  public enum CodingKeys: String, CodingKey {
    case citation_key
    case idx
  }

  public init(from decoder: Decoder) throws {
    let container = try decoder.container(keyedBy: CodingKeys.self)
    let idx = try container.decode(Int.self, forKey: .idx)
    let citationKey = try container.decode(String.self, forKey: .citation_key)

    self.init(citationKey: citationKey, idx: UInt8(idx))
  }

  public func encode(to encoder: any Encoder) throws {
    var container = encoder.container(keyedBy: CodingKeys.self)

    try container.encode(idx, forKey: .idx)
    try container.encode(citationKey, forKey: .citation_key)
  }
}
