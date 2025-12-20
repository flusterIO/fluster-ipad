//
//  empty_bib_search_results_view.swift
//  Fluster
//
//  Created by Andrew on 11/5/25.
//

import FlusterSwift
import SwiftData
import SwiftUI

struct EmptyMarkdownSearchResultsView: View {
    @Environment(ThemeManager.self) private var themeManager: ThemeManager
    @Binding var editingNote: NoteModel?

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
            Text("No notes found")
                .font(.title2)
            NavigationLink(
                destination: CreateNoteSheetView(
                    editingNote: $editingNote,
                    dismissOnSubmit: true
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
    EmptyMarkdownSearchResultsView(editingNote: .constant(nil))
}
