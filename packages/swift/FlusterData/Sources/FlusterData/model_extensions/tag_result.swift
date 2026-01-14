import FlusterSwiftMdxParser
import Foundation

extension TagResult: @retroactive Codable {
  public enum CodingKeys: String, CodingKey {
    case body
  }

  public init(from decoder: Decoder) throws {
    let container = try decoder.container(keyedBy: CodingKeys.self)
    let body = try container.decode(String.self, forKey: .body)
    self.init(body: body)
  }

  public func encode(to encoder: any Encoder) throws {
    var container = encoder.container(keyedBy: CodingKeys.self)
    try container.encode(body, forKey: .body)
  }
}
