//
//  File.swift
//  FlusterData
//
//  Created by Andrew on 3/8/26.
//

import Foundation

extension NoteSummary: Codable {
  public enum CodingKeys: String, CodingKey {
    case generationMethod
    case body
    case ctime
  }

  public convenience init(from decoder: Decoder) throws {
    let container = try decoder.container(keyedBy: CodingKeys.self)
    let generationMethod = try container.decode(
      NoteSummaryGenerationMethod.self, forKey: .generationMethod)
    let body = try container.decode(String.self, forKey: .body)
    let ctime = try container.decode(Date.self, forKey: .ctime)

    self.init(
      generationMethod: generationMethod, body: body, ctime: ctime
    )
  }

  public func encode(to encoder: any Encoder) throws {
    var container = encoder.container(keyedBy: CodingKeys.self)
    try container.encode(body, forKey: .body)
    try container.encode(ctime, forKey: .ctime)
    try container.encode(generationMethod, forKey: .generationMethod)
  }
}
