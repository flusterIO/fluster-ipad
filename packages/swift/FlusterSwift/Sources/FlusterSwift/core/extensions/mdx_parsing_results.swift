import FlusterRust
import Foundation

extension MdxParsingResult: Codable {
  public enum CodingKeys: String, CodingKey {
    case content
    case tags
    case front_matter
    case citations
    case dictionary_entries
  }

  public init(from decoder: Decoder) throws {
    let container = try decoder.container(keyedBy: CodingKeys.self)
    let content = try container.decode(String.self, forKey: .content)
    let tags = try container.decode([TagResult].self, forKey: .tags)
    let frontMatter = try container.decodeIfPresent(FrontMatterResult.self, forKey: .front_matter)
    let citations = try container.decode([CitationResult].self, forKey: .citations)
    let dictionaryEntries = try container.decode(
      [DictionaryEntryResult].self, forKey: .dictionary_entries)

    self.init(
      content: content, tags: tags, frontMatter: frontMatter, citations: citations,
      dictionaryEntries: dictionaryEntries)
  }

  public func encode(to encoder: any Encoder) throws {
    var container = encoder.container(keyedBy: CodingKeys.self)
    try container.encode(content, forKey: .content)
    try container.encode(tags, forKey: .tags)
    try container.encodeIfPresent(frontMatter, forKey: .front_matter)
    try container.encode(citations, forKey: .citations)
  }
}
