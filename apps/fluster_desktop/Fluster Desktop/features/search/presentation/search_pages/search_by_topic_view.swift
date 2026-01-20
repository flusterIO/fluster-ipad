//
//  search_by_topic_view.swift
//  Fluster
//
//  Created by Andrew on 1/19/26.
//

import SwiftUI
import FlusterData
import SwiftData

struct SearchByTopicView: View {
    let item: TopicModel
    @Query private var notes: [NoteModel]
    init(item: TopicModel) {
        self.item = item
        let itemValue = item.value
        _notes = Query(
            filter: #Predicate<NoteModel> { note in
                note.topic?.value == itemValue
            }
        )
    }
    var body: some View {
        SearchPageView(notes: notes)
            .navigationTitle("Search By Topic")
    }
}

#Preview {
    SearchByTopicView(
        item: TopicModel(value: "Calculus")
    )
}
