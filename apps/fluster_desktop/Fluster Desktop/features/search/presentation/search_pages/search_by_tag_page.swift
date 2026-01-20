//
//  search_by_tag_page.swift
//  Fluster
//
//  Created by Andrew on 1/19/26.
//

import SwiftUI
import FlusterData
import SwiftData

struct SearchByTagView: View {
    let item: TagModel
    @Query private var notes: [NoteModel]
    init(item: TagModel) {
        self.item = item
        let itemValue = item.value
        _notes = Query(
            filter: #Predicate<NoteModel>{ note in
                note.tags.contains(where: { $0.value == itemValue })
            }
        )
    }
    var body: some View {
        SearchPageView(notes: notes)
    }
}

#Preview {
    SearchByTagView(item: TagModel(value: "My Tag"))
}
