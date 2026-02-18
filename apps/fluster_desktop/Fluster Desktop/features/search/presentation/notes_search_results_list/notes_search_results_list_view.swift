//
//  notes_search_results_list_view.swift
//  Fluster
//
//  Created by Andrew on 2/18/26.
//

import SwiftUI
import FlusterData


// Use this as a wrapper for the SearchPageView for now, but implement a simpler UI later and leave the SearchPage as an independent view.
struct NoteSearchResultsListView: View {
    let notes: [NoteModel]
    var body: some View {
        SearchPageView(notes: notes)
    }
}

#Preview {
    NoteSearchResultsListView(notes: [])
}
