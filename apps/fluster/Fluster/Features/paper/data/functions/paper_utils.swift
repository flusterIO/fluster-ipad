//
//  paper_utils.swift
//  Fluster
//
//  Created by Andrew on 11/27/25.
//

import Foundation
import PencilKit

// TODO: Move this to some place that makes sense.
func loadDrawing(note: NoteModel) -> PKDrawing {
    do {
        // Recreate the PKDrawing from the Data
        return try PKDrawing(data: note.drawing)
    } catch {
        print("Error loading PKDrawing: \(error)")
        return PKDrawing()
    }
}
