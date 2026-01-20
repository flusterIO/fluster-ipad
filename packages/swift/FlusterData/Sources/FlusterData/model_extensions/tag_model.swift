//
//  File.swift
//  FlusterData
//
//  Created by Andrew on 1/20/26.
//

import Foundation

extension TagModel: Codable {
  public enum CodingKeys: String, CodingKey {
    case value
    case caseInsensitive
    case ctime
    case utime
    case lastAccess
  }

  public convenience init(from decoder: Decoder) throws {
    let container = try decoder.container(keyedBy: CodingKeys.self)
    let value = try container.decode(String.self, forKey: .value)
    let caseInsensitive = try container.decodeIfPresent(String.self, forKey: .caseInsensitive)
    let ctime = try container.decodeIfPresent(Date.self, forKey: .ctime)
    let utime = try container.decodeIfPresent(Date.self, forKey: .utime)
    let lastAccess = try container.decodeIfPresent(Date.self, forKey: .lastAccess)

    self.init(
      value: value,
      ctime: ctime ?? .now,
      utime: utime ?? .now,
      lastAccess: lastAccess ?? .now
    )
  }

  public func encode(to encoder: any Encoder) throws {
    var container = encoder.container(keyedBy: CodingKeys.self)
    try container.encode(value, forKey: .value)
    try container.encode(caseInsensitive, forKey: .caseInsensitive)
    try container.encode(ctime, forKey: .ctime)
    try container.encode(utime, forKey: .utime)
    try container.encode(lastAccess, forKey: .lastAccess)
  }
}

