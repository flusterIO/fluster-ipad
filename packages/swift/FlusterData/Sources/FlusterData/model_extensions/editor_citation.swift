//
//  File.swift
//  FlusterData
//
//  Created by Andrew on 3/5/26.
//

import FlatBuffers
import Foundation

extension EditorCitation {
    public func serializeToFlatBufferOffset(_ builder: inout FlatBufferBuilder) -> Offset {
      let htmlOffset = builder.create(string: self.html)
      let citationKeyOffset = builder.create(string: self.citation_key)
      let citationOffset = MdxSerialization_EditorCitationBuffer.createEditorCitationBuffer(
        &builder, citationKeyOffset: citationKeyOffset, htmlOffset: htmlOffset)
    return citationOffset
  }
}
