import ConundrumSwift
import Foundation

extension MdxParsingResult: @retroactive Codable {
  public enum CodingKeys: String, CodingKey {
    case note_id
    case content
    case tags
    case front_matter
    case orderedCitationKeys
    case dictionary_entries
    case outgoing_links
    case ignore_all_parsers
    case ai_secondary_parse_requests
    case success
    case toc
  }

  public init(from decoder: Decoder) throws {
    let container = try decoder.container(keyedBy: CodingKeys.self)
    let content = try container.decode(String.self, forKey: .content)
    let id = try container.decodeIfPresent(String.self, forKey: .note_id)
    let tags = try container.decode([TagResult].self, forKey: .tags)
    let frontMatter = try container.decodeIfPresent(FrontMatterResult.self, forKey: .front_matter)
    let citations = try container.decode([String].self, forKey: .orderedCitationKeys)
    let ignoreAllParsers = try container.decode(Bool.self, forKey: .ignore_all_parsers)
    let toc = try container.decode([MarkdownHeadingStringifiedResult].self, forKey: .toc)
    let outgoing_links = try container.decode(
      [NoteOutgoingLinkResult].self, forKey: .outgoing_links)
    let dictionaryEntries = try container.decode(
      [DictionaryEntryResult].self, forKey: .dictionary_entries)
    let aiParsingRequests = try container.decode(
      [AiSerializationRequestPhase1].self, forKey: .ai_secondary_parse_requests)
    let success = try container.decode(Bool.self, forKey: .success)

    self.init(
      noteId: id,
      content: content,
      tags: tags,
      frontMatter: frontMatter,
      orderedCitationKeys: citations,
      dictionaryEntries: dictionaryEntries,
      outgoingLinks: outgoing_links,
      toc: toc,
      ignoreAllParsers: ignoreAllParsers,
      aiSecondaryParseRequests: aiParsingRequests,
      success: success,
      error: nil
    )
  }

  public func encode(to encoder: any Encoder) throws {
    var container = encoder.container(keyedBy: CodingKeys.self)
    try container.encode(content, forKey: .content)
    try container.encode(tags, forKey: .tags)
    try container.encodeIfPresent(frontMatter, forKey: .front_matter)
    try container.encode(orderedCitationKeys, forKey: .orderedCitationKeys)
    try container.encode(aiSecondaryParseRequests, forKey: .ai_secondary_parse_requests)
    try container.encode(success, forKey: .success)
  }
}
