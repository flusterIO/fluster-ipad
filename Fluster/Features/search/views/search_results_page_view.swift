//
//  search_results_page_view.swift
//  Fluster
//
//  Created by Andrew on 10/29/25.
//

import SwiftUI

struct SearchPageView: View {
    @State private var inputValue: String = ""
    @State private var selectedCategory: SearchCategory = .note
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
                                SearchCategoryRow(item: cat)
                            }
                        )
                    }
                    Spacer()
                }
            },
            detail: {
                switch selectedCategory {
                case .note:
                    Text("Note")
                case .citation:
                    Text("Bibliography")
                }
            }
        )
    }
}

#Preview {
    SearchPageView()
        .environment(ThemeManager(initialTheme: FlusterDark()))
}
