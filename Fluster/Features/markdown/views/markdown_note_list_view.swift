//
//  markdown_note_list_view.swift
//  Fluster
//
//  Created by Andrew on 10/29/25.
//

import SwiftUI
import SwiftData

struct MarkdownNoteListView: View {
    @Query var notes: [MarkdownNote]
    var body: some View {
        NavigationStack {
            List {
                ForEach(notes) { note in
                    VStack(alignment: .center){
                        Text(note.label)
                            .font(.headline)
                    }
                }
        }
        }
    }
}

#Preview {
    MarkdownNoteListView()
}
