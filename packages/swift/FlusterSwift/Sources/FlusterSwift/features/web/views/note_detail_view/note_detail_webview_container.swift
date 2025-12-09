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

        let citationsVectorOffset: [Offset] = []

        for citation in note.citations {
            let citationOffset =
                MdxSerialization_CitationResultBuffer.createCitationResultBuffer(
                    &builder,
                    citationKeyOffset: builder.create(
                        string: citation.citationKey
                    ),
                    bodyOffset: builder.create(string: citation.data)
                )
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
                        for: note.last_read,
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
            """)
    }
}
