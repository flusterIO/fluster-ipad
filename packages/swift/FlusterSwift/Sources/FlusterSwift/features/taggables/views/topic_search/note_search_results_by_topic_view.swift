//
//  File.swift
//  FlusterSwift
//
//  Created by Andrew on 12/9/25.
//

import SwiftUI
import SwiftData

struct NoteSearchResultsByTopicView: View {
    @Query(sort: \NoteModel.last_read, order: .reverse) private var notes:
        [NoteModel]
    @State private var noteQuery: String = ""
    @Binding var editingNote: NoteModel?
    let topic: TopicModel
    var filteredNotes: [NoteModel] {
        return noteQuery.isEmpty
            ? notes
            : MdxTextUtils.sortNotesByMarkdownBodyMatch(
                notes: notes,
                query: noteQuery,
                filterNoMatch: true
            )
    }
    
    public init(topic: TopicModel, editingNote: Binding<NoteModel?>) {
        self._editingNote = editingNote
        self.topic = topic
        let queryValue = topic.value
        _notes = Query(
            filter: #Predicate<NoteModel> { note in
                note.tags.contains(where: { $0.value == queryValue })
            },
            sort: [SortDescriptor(\NoteModel.last_read, order: .reverse)],
            animation: .default
        )
    }

    
    var body: some View {
        ZStack {
            List(filteredNotes) { note in
                NoteSearchResultItemView(item: note, editingNote: $editingNote)
            }
            if filteredNotes.isEmpty {
                NoNotesFoundView()
                    .searchable(text: $noteQuery, prompt: "Search notes")
                    .navigationTitle("Notes")
            }
        }
        .searchable(text: $noteQuery, prompt: "Search notes")
        .navigationTitle("Notes")
    }
}

#Preview {
    NoteSearchResultsByTopicView(
        topic: TopicModel(value: "math"),
        editingNote: .constant(nil)
    )
}
