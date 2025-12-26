//
//  File.swift
//  FlusterSwift
//
//  Created by Andrew on 12/25/25.
//

import FlusterRust
import Foundation

extension DictionaryEntryResult: Codable {
  public enum CodingKeys: String, CodingKey {
    case label
    case body
    case id
  }

  public init(from decoder: Decoder) throws {
    let container = try decoder.container(keyedBy: CodingKeys.self)
    let label = try container.decode(String.self, forKey: .label)
    let body = try container.decode(String.self, forKey: .body)
    let id = try container.decode(String.self, forKey: .id)

    self.init(
      label: label,
      body: body
    )
  }

  public func encode(to encoder: any Encoder) throws {
    var container = encoder.container(keyedBy: CodingKeys.self)

    try container.encode(body, forKey: .body)
    try container.encode(label, forKey: .label)
  }
}
