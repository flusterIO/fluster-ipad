//
//  mdx_utility_structs.swift
//  Fluster
//
//  Created by Andrew on 11/27/25.
//

import Foundation

public struct MdxTitleResult {
  public let depth: Int
  public let content: String
  public init(depth: Int, content: String) {
    self.depth = depth
    self.content = content
  }
}
