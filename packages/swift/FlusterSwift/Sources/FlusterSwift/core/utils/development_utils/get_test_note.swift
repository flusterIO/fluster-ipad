//
//  File.swift
//  FlusterSwift
//
//  Created by Andrew on 12/11/25.
//

import Foundation

public func getTestNote() -> NoteModel {
    return NoteModel(
        markdown: MarkdownNote(
            body: """
                # My test note
                """,
            summary: nil
        )
    )
}
