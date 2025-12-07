//
//  File.swift
//  FlusterSwift
//
//  Created by Andrew on 12/6/25.
//

import FlatBuffers
import Foundation

extension Data {
    func toMdxParsingResult() -> MdxSerialization_MdxParsingResultBuffer? {
        let byteArray: [UInt8] = Array(self)
        var buf = ByteBuffer.init(bytes: byteArray)
        let mdxRes: MdxSerialization_MdxParsingResultBuffer? =
            try? getCheckedRoot(byteBuffer: &buf)
        return mdxRes
    }
}
