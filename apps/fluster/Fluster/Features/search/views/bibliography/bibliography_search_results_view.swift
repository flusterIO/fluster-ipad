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
    @Binding var activeCategory: SearchCategoryId
    @Query(sort: \BibEntryModel.ctime) var bibEntries: [BibEntryModel]
    
    
    var body: some View {
        if bibEntries.isEmpty {
            EmptyBibSearchResultsView(activeCategory: $activeCategory)
        } else {
            List {
                ForEach(bibEntries, id: \.id) { item in
                    BibEntrySearchResultItemView(item: item)
                }
            }
            .navigationTitle("Search by citation")
        }
    }
}

#Preview {
    BibliographySearchResultsView(searchQuery: .constant(""), activeCategory: .constant(.createNote))
}
