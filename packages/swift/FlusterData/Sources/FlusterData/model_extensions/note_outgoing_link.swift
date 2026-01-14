//
//  File.swift
//  FlusterSwift
//
//  Created by Andrew on 1/9/26.
//

import FlusterSwiftMdxParser
import Foundation

extension NoteOutgoingLinkResult: @retroactive Codable {
  public enum CodingKeys: String, CodingKey {
    case link_to_note_id
  }

  public init(from decoder: Decoder) throws {
    let container = try decoder.container(keyedBy: CodingKeys.self)
    let link_to_note_id = try container.decode(String.self, forKey: .link_to_note_id)
    self.init(
      linkToNoteId: link_to_note_id
    )
  }

  public func encode(to encoder: any Encoder) throws {
    var container = encoder.container(keyedBy: CodingKeys.self)

    try container.encode(linkToNoteId, forKey: .link_to_note_id)
  }
}
