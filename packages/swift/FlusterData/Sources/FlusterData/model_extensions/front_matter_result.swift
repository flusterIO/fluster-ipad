import FlusterSwiftMdxParser
import Foundation

extension FrontMatterResult: @retroactive Codable {
  public enum CodingKeys: String, CodingKey {
    case ignored_parsers,
      title,
      user_defined_id,
      file_path,
      topic,
      subject
  }

  public init(from decoder: Decoder) throws {
    let container = try decoder.container(keyedBy: CodingKeys.self)
    let ignoredParsers = try container.decodeIfPresent(
      [String].self,
      forKey: .ignored_parsers
    )
    let title = try container.decodeIfPresent(String.self, forKey: .title)
    let topic = try container.decodeIfPresent(String.self, forKey: .topic)
    let subject = try container.decodeIfPresent(String.self, forKey: .subject)
    let filePath = try container.decodeIfPresent(String.self, forKey: .file_path)
    let userDefinedId = try container.decodeIfPresent(
      String.self,
      forKey: .user_defined_id
    )

    self.init(
      ignoredParsers: ignoredParsers ?? [],
      title: title,
      userDefinedId: userDefinedId,
      filePath: filePath,
      topic: topic,
      subject: subject,
    )
  }

  public func encode(to encoder: any Encoder) throws {
    var container = encoder.container(keyedBy: CodingKeys.self)
    try container.encode(ignoredParsers, forKey: .ignored_parsers)
    try container.encode(title, forKey: .title)
    try container.encode(userDefinedId, forKey: .user_defined_id)
  }
}
