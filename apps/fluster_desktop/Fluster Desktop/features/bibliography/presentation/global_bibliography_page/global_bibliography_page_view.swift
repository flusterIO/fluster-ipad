//
//  global_bibliography_page_view.swift
//  Fluster
//
//  Created by Andrew on 1/18/26.
//

import SwiftUI
import FlusterData
import SwiftData

struct GlobalBibliographyPageView: View {
    @Query(sort: \BibEntryModel.title, order: .forward) var entries: [BibEntryModel]
    var body: some View {
        BibliographyEntryListView(entries: entries)
    }
}

#Preview {
    GlobalBibliographyPageView()
}
