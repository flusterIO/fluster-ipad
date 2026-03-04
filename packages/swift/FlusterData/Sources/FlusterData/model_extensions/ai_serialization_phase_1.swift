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
    case createNoteSpecificStudyGuide
  }
  public init(from decoder: Decoder) throws {
    let container = try decoder.singleValueContainer()
    let stored = try container.decode(MatchingStringKeys.self)
    switch stored {
      case MatchingStringKeys.createNoteSpecificStudyGuide:
        self = .createNoteSpecificStudyGuide
    }
  }

  public func encode(to encoder: Encoder) throws {
    var container = encoder.singleValueContainer()
    switch self {
      case .createNoteSpecificStudyGuide:
        try container.encode(MatchingStringKeys.createNoteSpecificStudyGuide)
    }
  }
}

extension AiSerializationRequestPhase1: @retroactive Codable {
  public enum CodingKeys: String, CodingKey {
    case requestType
  }

  public init(from decoder: Decoder) throws {
    let container = try decoder.container(keyedBy: CodingKeys.self)
    let requestType = try container.decode(AiSerializationRequestType.self, forKey: .requestType)
    self.init(requestType: requestType)
  }

  public func encode(to encoder: Encoder) throws {
    var container = encoder.container(keyedBy: CodingKeys.self)
    try container.encode(self.requestType, forKey: .requestType)
  }
}
