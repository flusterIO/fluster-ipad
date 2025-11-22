//
//  empty_bib_search_results_view.swift
//  Fluster
//
//  Created by Andrew on 11/5/25.
//

import SwiftUI

struct EmptyBibSearchResultsView: View {
    @Environment(ThemeManager.self) private var themeManager: ThemeManager
    @Binding var activeCategory: SearchCategoryId

    var body: some View {
        VStack(spacing: 16) {
            ZStack {
                themeManager.theme.primary
                    .frame(width: 64, height: 64)
                    .clipShape(Circle())
                Image(systemName: "magnifyingglass")
                    .imageScale(.large)
                    .foregroundStyle(themeManager.theme.primary_foreground)
            }
            Text("No entries found")
                .font(.title2)
            Button("Create") {
                activeCategory = .createNote
            }
            .buttonStyle(.bordered)
        }
    }
}

#Preview {
    EmptyBibSearchResultsView(activeCategory: .constant(.createNote))
}
