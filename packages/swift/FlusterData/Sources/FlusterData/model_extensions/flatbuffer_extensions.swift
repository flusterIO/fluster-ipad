//
//  File.swift
//  FlusterData
//
//  Created by Andrew on 2/17/26.
//

import FlatBuffers
import Foundation

public extension FlatBufferBuilder {
  public func toBytesArray() -> [UInt8] {
    let bytes: [UInt8] = Array(self.data)
    return bytes
  }
}
