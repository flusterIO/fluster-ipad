//
//  editing_note_is_valid.swift
//  Fluster
//
//  Created by Andrew on 2/11/26.
//

import FlusterData
import Foundation
import SwiftData

public func editingNoteIsValid(note: NoteModel, appState: AppState) -> Bool {
    return !note.isDeleted && (note.id == appState.editingNoteId)
}
