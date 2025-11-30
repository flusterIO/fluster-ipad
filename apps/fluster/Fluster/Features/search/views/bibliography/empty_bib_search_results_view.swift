//
//  empty_bib_search_results_view.swift
//  Fluster
//
//  Created by Andrew on 11/5/25.
//

import SwiftUI
import FlusterSwift

struct EmptyBibSearchResultsView: View {
    @Environment(ThemeManager.self) private var themeManager: ThemeManager
    let bibtexEditorContainer: BibtexEditorWebviewContainer

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
            NavigationLink(
                destination: CreateBibEntrySheetView(
                    editingBibEntry: .constant(nil),
                    ignoreEditingNote: true,
                    container: bibtexEditorContainer
                ),
                label: {
                    Label("Create", systemImage: "plus")
                }
            )
            .buttonStyle(.borderedProminent)
        }
    }
}

#Preview {
    EmptyBibSearchResultsView(
        bibtexEditorContainer: BibtexEditorWebviewContainer(bounce: true, scrollEnabled: true)
    )
}
