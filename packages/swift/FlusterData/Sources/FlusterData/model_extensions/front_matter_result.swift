import FlusterSwiftMdxParser
import Foundation

extension FrontMatterResult: @retroactive Codable {
  public enum CodingKeys: String, CodingKey {
    case ignored_parsers,
      title,
      user_defined_id
  }

  public init(from decoder: Decoder) throws {
    let container = try decoder.container(keyedBy: CodingKeys.self)
    let ignoredParsers = try container.decodeIfPresent(
      [String].self,
      forKey: .ignored_parsers
    )
    let title = try container.decodeIfPresent(String.self, forKey: .title)
    let userDefinedId = try container.decodeIfPresent(
      String.self,
      forKey: .user_defined_id
    )

    self.init(
      ignoredParsers: ignoredParsers ?? [],
      title: title,
      userDefinedId: userDefinedId
    )
  }

  public func encode(to encoder: any Encoder) throws {
    var container = encoder.container(keyedBy: CodingKeys.self)
    try container.encode(ignoredParsers, forKey: .ignored_parsers)
    try container.encode(title, forKey: .title)
    try container.encode(userDefinedId, forKey: .user_defined_id)
  }
}
