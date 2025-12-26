//
//  File.swift
//  FlusterSwift
//
//  Created by Andrew on 12/6/25.
//

import FlatBuffers
import Foundation

extension Data {
  public func toMdxParsingResult() -> MdxSerialization_MdxParsingResultBuffer? {
    do {
      let byteArray: [UInt8] = Array(self)
      var buf = ByteBuffer.init(bytes: byteArray)
      let mdxRes: MdxSerialization_MdxParsingResultBuffer? =
        try getCheckedRoot(byteBuffer: &buf)
      return mdxRes
    } catch {
      print("Error: \(error.localizedDescription)")
      return nil
    }
  }
}
