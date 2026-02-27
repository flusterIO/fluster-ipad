//
//  File.swift
//  FlusterData
//
//  Created by Andrew on 1/14/26.
//

import Foundation

extension [BibEntryModel] {
  /// Remove all occurences of an id in the array.
  public mutating func removeById(id: String) {
    self.removeAll(where: { $0.id == id })
  }
  public mutating func removeByCitationKey(citationKey: String) {
    self.removeAll(where: { $0.citationKey == citationKey })
  }
  /// Safely append an item without producing duplicates. If an item with the id exists it will be replaced, overwriting
  /// the bibliography entry with that same citationKey.
  public mutating func appendWithoutDuplicates(item: BibEntryModel) {
    if let citationKey = item.citationKey {
      self.removeByCitationKey(citationKey: citationKey)
      self.append(item)
    }
  }
}

extension BibEntryModel: Codable {
  public enum CodingKeys: String, CodingKey {
    case id, citationKey, data, ctime, utime, lastAccess
  }

  public convenience init(from decoder: Decoder) throws {
    let container = try decoder.container(keyedBy: CodingKeys.self)
    let id = try container.decodeIfPresent(String.self, forKey: .id)
    let citationKey = try container.decode(String.self, forKey: .citationKey)
    let data = try container.decode(String.self, forKey: .data)
    let ctime = try container.decode(Date.self, forKey: .ctime)
    let utime = try container.decode(Date.self, forKey: .utime)
    let lastAccess = try container.decode(Date.self, forKey: .lastAccess)

    self.init(id: id, data: data, ctime: ctime, utime: utime, lastAccess: lastAccess, notes: [])
  }

  public func encode(to encoder: any Encoder) throws {
    var container = encoder.container(keyedBy: CodingKeys.self)
    try container.encode(id, forKey: .id)
    try container.encode(citationKey, forKey: .citationKey)
    try container.encode(data, forKey: .data)
    try container.encode(ctime, forKey: .ctime)
    try container.encode(utime, forKey: .utime)
    try container.encode(lastAccess, forKey: .lastAccess)
  }
}
