//
//  File.swift
//  FlusterSwift
//
//  Created by Andrew on 12/25/25.
//

import FlatBuffers
import Foundation
import FlusterData


#if os(iOS)
public final class DictionaryWebviewContainer: WebviewContainer<DictionaryWebviewEvents> {
  public func setDictionaryContent(entries: [DictionaryEntryModel]) {
    var builder = FlatBufferBuilder()
    let entries_offset: [Offset] = entries.map({ dict in
      let idOffset = builder.create(string: dict.id)
      let labelOffset = builder.create(string: dict.label)
      let bodyOffset = builder.create(string: dict.body)
      let entry_offset = Dictionary_DictionaryEntryResultBuffer.createDictionaryEntryResultBuffer(
        &builder, labelOffset: labelOffset, bodyOffset: bodyOffset)
      return entry_offset
    })
    let vector_offset = builder.createVector(ofOffsets: entries_offset)
    let data = Dictionary_DictionaryData.createDictionaryData(
      &builder, entriesVectorOffset: vector_offset)
    builder.finish(offset: data)
    let bytes: [UInt8] = Array(builder.data)
    self.runJavascript(
      """
      window.localStorage.setItem("\(DictionaryWebviewStorageKeys.dictionaryData.rawValue)", `\(bytes)`);
      window.dispatchEvent(new CustomEvent("\(DictionaryWebviewEvents.setDictionaryData.rawValue)", {
          detail: \(bytes)
      }))
      """
    )
  }
}
#endif
