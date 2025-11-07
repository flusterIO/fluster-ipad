//
//  bibliography_search_results_view.swift
//  Fluster
//
//  Created by Andrew on 11/5/25.
//

import SwiftUI
import SwiftData

struct BibliographySearchResultsView: View {
    @Binding var searchQuery: String
    @Query var notes: [MarkdownNote]
    @Binding var activeCategory: SearchCategoryId


    var body: some View {
        if notes.isEmpty {
            EmptyBibSearchResultsView(activeCategory: $activeCategory)
        } else {
            Text("Bibliography stuff here dawg")
        }
    }
}

#Preview {
    BibliographySearchResultsView(searchQuery: .constant(""), activeCategory: .constant(.createNote))
}
