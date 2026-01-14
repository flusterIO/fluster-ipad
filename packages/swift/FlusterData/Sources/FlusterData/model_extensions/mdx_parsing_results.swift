import FlusterSwiftMdxParser
import Foundation

extension MdxParsingResult: @retroactive Codable {
  public enum CodingKeys: String, CodingKey {
    case note_id
    case content
    case tags
    case front_matter
    case citations
    case dictionary_entries
    case outgoing_links
  }

  public init(from decoder: Decoder) throws {
    let container = try decoder.container(keyedBy: CodingKeys.self)
    let content = try container.decode(String.self, forKey: .content)
    let id = try container.decode(String.self, forKey: .note_id)
    let tags = try container.decode([TagResult].self, forKey: .tags)
    let frontMatter = try container.decodeIfPresent(FrontMatterResult.self, forKey: .front_matter)
    let citations = try container.decode([CitationResult].self, forKey: .citations)
    let outgoing_links = try container.decode(
      [NoteOutgoingLinkResult].self, forKey: .outgoing_links)
    let dictionaryEntries = try container.decode(
      [DictionaryEntryResult].self, forKey: .dictionary_entries)

    self.init(
      noteId: id,
      content: content, tags: tags, frontMatter: frontMatter, citations: citations,
      dictionaryEntries: dictionaryEntries, outgoingLinks: outgoing_links
    )
  }

  public func encode(to encoder: any Encoder) throws {
    var container = encoder.container(keyedBy: CodingKeys.self)
    try container.encode(content, forKey: .content)
    try container.encode(tags, forKey: .tags)
    try container.encodeIfPresent(frontMatter, forKey: .front_matter)
    try container.encode(citations, forKey: .citations)
  }
}
