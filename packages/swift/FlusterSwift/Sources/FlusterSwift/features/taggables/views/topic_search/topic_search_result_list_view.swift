//
//  File.swift
//  FlusterSwift
//
//  Created by Andrew on 12/9/25.
//

import SwiftData
import SwiftUI

public struct TopicSearchResultListView: View {
    @Query(sort: \TopicModel.lastAccess, order: .reverse) private var topics:
        [TopicModel]
    @Binding var topicQuery: String
    @Binding var editingNote: NoteModel?
    public init(topicQuery: Binding<String>, editingNote: Binding<NoteModel?>) {
        self._topicQuery = topicQuery
        self._editingNote = editingNote
        if !topicQuery.wrappedValue.isEmpty {
            let t = topicQuery.wrappedValue
            _topics = Query(
                filter: #Predicate<TopicModel> { topic in
                    topic.value.localizedStandardContains(t)
                },
                sort: [
                    SortDescriptor(\TopicModel.lastAccess, order: .reverse)
                ],
                animation: .default
            )
        }
    }
    public var body: some View {
        if (topics.isEmpty && topicQuery.isEmpty) {
            NoNotesFoundView(title: "No topics found", subtitle: "Add a topic to your note to filter your notes by topic here.")
        } else {
            List(topics) { topic in
                NavigationLink(
                    destination: {
                        NoteSearchResultsByTopicView(topic: topic, editingNote: $editingNote)
                    },
                    label: {
                        Text(topic.value)
                    })
            }
            .searchable(text: $topicQuery, prompt: "Search Topics")
            .navigationTitle("Topics")
        }
    }
}

#Preview {
    TopicSearchResultListView(
        topicQuery: .constant(""),
        editingNote: .constant(nil)
    )
}
