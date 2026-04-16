//
//  File.swift
//  FlusterData
//
//  Created by Andrew on 3/4/26.
//

import ConundrumSwift
import Foundation

extension AiSerializationRequestType: @retroactive Codable {
  enum MatchingStringKeys: String, Codable {
    case createNoteSpecificStudyGuide, recommendSearch, summarizeNote
  }

  public init(from decoder: Decoder) throws {
    let container = try decoder.singleValueContainer()
    let stored = try container.decode(MatchingStringKeys.self)
    switch stored {
      case MatchingStringKeys.createNoteSpecificStudyGuide:
        self = .createNoteSpecificStudyGuide
      case MatchingStringKeys.recommendSearch:
        self = .recommendResearch
      case MatchingStringKeys.summarizeNote:
        self = .summarizeNote
    }
  }

  public func encode(to encoder: Encoder) throws {
    var container = encoder.singleValueContainer()
    switch self {
      case .createNoteSpecificStudyGuide:
        try container.encode(MatchingStringKeys.createNoteSpecificStudyGuide)
      case .recommendResearch:
        try container.encode(MatchingStringKeys.recommendSearch)
      case .summarizeNote:
        try container.encode(MatchingStringKeys.summarizeNote)
    }
  }
}

extension ParsedCodeBlock: @retroactive Codable {
  public enum CodingKeys: String, CodingKey {
    case fullMatch,
      language,
      content,
      metaData,
      depth
  }
  public init(from decoder: Decoder) throws {
    let container = try decoder.container(keyedBy: CodingKeys.self)
    let fullMatch = try container.decode(String.self, forKey: .fullMatch)
    let langTag = try container.decode(SupportedCodeBlockSyntax.self, forKey: .language)
    let blockContent = try container.decode(String.self, forKey: .content)
    let metaData = try container.decode(String.self, forKey: .metaData)
    let depth = try container.decode(UInt8.self, forKey: .depth)
    self.init(
      language: langTag, metaData: metaData, depth: depth, content: blockContent,
      fullMatch: fullMatch
    )
  }

  public func encode(to encoder: Encoder) throws {
    var container = encoder.container(keyedBy: CodingKeys.self)
    try container.encode(self.content, forKey: .content)
    try container.encode(self.fullMatch, forKey: .fullMatch)
    try container.encode(self.language, forKey: .language)
    try container.encode(self.metaData, forKey: .metaData)
    try container.encode(self.depth, forKey: .depth)
  }
}

extension AiSerializationRequestPhase1: @retroactive Codable {
  public enum CodingKeys: String, CodingKey {
    case parsingResult
  }

  public init(from decoder: Decoder) throws {
    let container = try decoder.container(keyedBy: CodingKeys.self)
    let codeBlockResult = try container.decode(
      ConundrumSwift.ParsedCodeBlock.self, forKey: .parsingResult)
    self.init(parsingResult: codeBlockResult)
  }

  public func encode(to encoder: Encoder) throws {
    var container = encoder.container(keyedBy: CodingKeys.self)
    try container.encode(self.parsingResult, forKey: .parsingResult)
  }
}
