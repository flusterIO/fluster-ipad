//
//  markdown_note_list_view.swift
//  Fluster
//
//  Created by Andrew on 10/29/25.
//

import SwiftUI
import SwiftData

struct MarkdownNoteListView: View {
    @Query var notes: [NoteModel]
    var body: some View {
        Text("Here")
    }
}

#Preview {
    MarkdownNoteListView()
}
