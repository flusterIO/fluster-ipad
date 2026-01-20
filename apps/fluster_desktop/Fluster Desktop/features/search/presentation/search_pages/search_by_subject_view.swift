//
//  search_by_subject_view.swift
//  Fluster
//
//  Created by Andrew on 1/19/26.
//

import SwiftUI
import FlusterData
import SwiftData


struct SearchBySubjectView: View {
    let item: SubjectModel
    @Query private var notes: [NoteModel]
    init(item: SubjectModel) {
        self.item = item
        let itemValue = item.value
        _notes = Query(
            filter: #Predicate<NoteModel> { note in
                note.subject?.value == itemValue
            }
        )
    }
    var body: some View {
        SearchPageView(notes: notes)
            .navigationTitle("Search By Subject")
    }
}

#Preview {
    SearchBySubjectView(
        item: SubjectModel(value: "Calculus")
    )
}
