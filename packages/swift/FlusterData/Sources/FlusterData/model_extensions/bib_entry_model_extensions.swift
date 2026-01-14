//
//  File.swift
//  FlusterData
//
//  Created by Andrew on 1/14/26.
//

import Foundation

extension [BibEntryModel] {
  /// Remove all occurences of an id in the array.
  mutating public func removeById(id: String) {
    self.removeAll(where: { $0.id == id })
  }
  /// Safely append an item without producing duplicates. If an item with the id exists it will be replaced.
  mutating public func appendWithoutDuplicates(item: BibEntryModel) {
    self.removeById(id: item.id)
    self.append(item)
  }
}
