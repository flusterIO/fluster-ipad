//
//  bibliography_page_view.swift
//  Fluster
//
//  Created by Andrew on 10/29/25.
//

import SwiftUI
import SwiftData

struct BibliographyPageView: View {
    @Query var bibEntries: [BibEntryModel]
    @State private var editing: BibEntryModel? = nil
    var body: some View {
        if bibEntries.isEmpty {
            EmptyBibListView()
        } else {
            BibListView(items: bibEntries, editing: $editing)
        }
    }
}

#Preview {
    BibliographyPageView()
}
