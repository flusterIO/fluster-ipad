//
//  File.swift
//  FlusterData
//
//  Created by Andrew on 3/4/26.
//

import FlusterSwiftMdxParser
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

extension FlusterSwiftMdxParser.CodeBlockParsingResult: @retroactive Codable {
  public enum CodingKeys: String, CodingKey {
    case fullMatch,
      languageTag,
      blockContent,
      metaData
  }
  public init(from decoder: Decoder) throws {
    let container = try decoder.container(keyedBy: CodingKeys.self)
    let fullMatch = try container.decode(String.self, forKey: .fullMatch)
    let langTag = try container.decode(String.self, forKey: .languageTag)
    let blockContent = try container.decode(String.self, forKey: .blockContent)
    let metaData = try container.decode(String.self, forKey: .metaData)
    self.init(
      fullMatch: fullMatch, languageTag: langTag, blockContent: blockContent, metaData: metaData
    )
  }

  public func encode(to encoder: Encoder) throws {
    var container = encoder.container(keyedBy: CodingKeys.self)
    try container.encode(self.blockContent, forKey: .blockContent)
    try container.encode(self.fullMatch, forKey: .fullMatch)
    try container.encode(self.languageTag, forKey: .languageTag)
    try container.encode(self.metaData, forKey: .metaData)
  }
}

extension AiSerializationRequestPhase1: @retroactive Codable {
  public enum CodingKeys: String, CodingKey {
    case parsingResult
  }

  public init(from decoder: Decoder) throws {
    let container = try decoder.container(keyedBy: CodingKeys.self)
    let codeBlockResult = try container.decode(
      FlusterSwiftMdxParser.CodeBlockParsingResult.self, forKey: .parsingResult)
    self.init(parsingResult: codeBlockResult)
  }

  public func encode(to encoder: Encoder) throws {
    var container = encoder.container(keyedBy: CodingKeys.self)
    try container.encode(self.parsingResult, forKey: .parsingResult)
  }
}
