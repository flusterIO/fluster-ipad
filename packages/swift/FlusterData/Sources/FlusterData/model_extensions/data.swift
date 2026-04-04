//
//  File.swift
//  FlusterSwift
//
//  Created by Andrew on 12/6/25.
//

import FlatBuffers
import Foundation
import FlusterData

extension Data {
  public func toMdxParsingResult() -> MdxSerialization_ParsedMdxDataTypescriptSafe? {
    do {
      let byteArray: [UInt8] = Array(self)
      var buf = ByteBuffer.init(bytes: byteArray)
      let mdxRes: MdxSerialization_ParsedMdxDataTypescriptSafe? =
        try getCheckedRoot(byteBuffer: &buf)
      return mdxRes
    } catch {
      print("Error converting to parsing result: \(error.localizedDescription)")
      return nil
    }
  }
}
