//
//  search_results_page_view.swift
//  Fluster
//
//  Created by Andrew on 10/29/25.
//

import SwiftUI

struct SearchPageView: View {
    @State private var inputValue: String = ""
    @State private var selectedCategory: SearchCategoryId = .note
    @Binding var editingNoteId: String?
    @Environment(ThemeManager.self) private var themeManager: ThemeManager

    var body: some View {
        NavigationSplitView(
            sidebar: {
                Text("Sidebar")
            },
            content: {
                VStack {
                    ForEach(searchCategoryItems) { cat in
                        Button(
                            action: {
                                selectedCategory = cat.id
                            },
                            label: {
                                SearchCategoryRow(item: cat, activeCategory: $selectedCategory)
                            }
                        )
                    }
                    Spacer()
                }
            },
            detail: {
                switch selectedCategory {
                case .note:
                    MarkdownNotesSearchResultsView(searchQuery: $inputValue, activeCategory: $selectedCategory)
                case .citation:
                    BibliographySearchResultsView(searchQuery: $inputValue, activeCategory: $selectedCategory)
                case .createNote:
                    CreateNoteSheetView(editingNoteId: $editingNoteId)
                }
            }
        )
    }
}

#Preview {
    SearchPageView(editingNoteId: .constant(nil))
        .environment(ThemeManager(initialTheme: FlusterDark()))
}
