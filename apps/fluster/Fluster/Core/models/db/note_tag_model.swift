//
//  note_tag_model.swift
//  Fluster
//
//  Created by Andrew on 11/2/25.
//

import SwiftData
import Foundation

@Model
class NoteTag {
    var noteId: UUID
    var tagValue: String
    init(noteId: UUID, tagValue: String) {
        self.noteId = noteId
        self.tagValue = tagValue
    }
}
