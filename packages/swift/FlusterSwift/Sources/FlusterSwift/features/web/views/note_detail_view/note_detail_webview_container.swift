//
//  File.swift
//  FlusterSwift
//
//  Created by Andrew on 12/5/25.
//

import Combine
import FlatBuffers
import SwiftUI
import WebKit
import FlusterData

#if os(iOS)
@MainActor
public final class NoteDetailWebviewContainer: WebviewContainer<
  NoteDetailWebviewEvents
>
{
  public func setNoteDetails(note: NoteModel) {
    var builder = FlatBufferBuilder(initialSize: 1024)
    var noteIdOffset = builder.create(string: note.id)
    var titleOffset = builder.create(
      string: note.markdown.title ?? "No title found"
    )
    var tagVectorOffset: [Offset] = []

    for t in note.tags {
      let x = MdxSerialization_TagResultBuffer.createTagResultBuffer(
        &builder,
        bodyOffset: builder.create(string: t.value)
      )
      tagVectorOffset.append(x)
    }

    var citationsVectorOffset: [Offset] = []

    for (idx, citation) in note.citations.enumerated() {
      let citationOffset =
        MdxSerialization_NoteDetails_NoteDetailCitationBuffer.createNoteDetailCitationBuffer(
          &builder,
          idOffset: builder.create(string: citation.citationKey ?? citation.id),
          bodyOffset: builder.create(string: citation.data),
          idx: UInt8(idx)
        )
        citationsVectorOffset.append(citationOffset)
    }

    let dateFormatter = RelativeDateTimeFormatter()
    dateFormatter.unitsStyle = .full
    dateFormatter.dateTimeStyle = .named
    let details =
      MdxSerialization_NoteDetails_NoteDetailDataBuffer
      .createNoteDetailDataBuffer(
        &builder,
        noteIdOffset: noteIdOffset,
        titleOffset: titleOffset,
        summaryOffset: builder.create(string: note.markdown.summary),
        topicOffset: builder.create(
          string: note.topic?.value
        ),
        subjectOffset: builder.create(
          string: note.subject?.value
        ),
        tagsVectorOffset: builder.createVector(
          ofOffsets: tagVectorOffset
        ),
        citationsVectorOffset: builder.createVector(
          ofOffsets: citationsVectorOffset
        ),
        lastModifiedStringOffset: builder.create(
          string: dateFormatter.localizedString(
            for: note.utime,
            relativeTo: .now
          )
        ),
        lastReadStringOffset: builder.create(
          string: dateFormatter.localizedString(
            for: note.lastRead,
            relativeTo: .now
          )
        )
      )
    builder.finish(offset: details)
    let bytes: [UInt8] = Array(builder.data)
    self.runJavascript(
      """
      window.dispatchEvent(
          new CustomEvent("\(NoteDetailWebviewEvents.setNoteDetails.rawValue)", {
              detail: \(bytes),
          }),
      );
      """
    )
  }

  public func setInitialData(
    colorScheme: ColorScheme,
    webviewTheme: WebViewTheme,
    fontSize: WebviewFontSize,
    note: NoteModel
  ) {
    self.applyWebViewColorScheme(
      darkMode: colorScheme == .dark
    )
    self.setWebviewTheme(
      theme: webviewTheme
    )
    self.setWebviewFontSize(
      fontSize: fontSize
    )
    self.setNoteDetails(note: note)
  }
}
#endif
