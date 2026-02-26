//
//  File.swift
//  FlusterSwift
//
//  Created by Andrew on 2/15/26.
//

import Foundation

struct Message: Codable {
  let content: String
}

extension String {
  func trunc(length: Int, trailing: String = "...") -> String {
    let maxLength = length - trailing.count
    guard maxLength > 0, !self.isEmpty, self.count > length else {
      return self
    }
    return String(self.prefix(maxLength)) + trailing
  }

}
