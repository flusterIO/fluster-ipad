//
//  empty_bib_list_view.swift
//  Fluster
//
//  Created by Andrew on 11/3/25.
//

import SwiftData
import SwiftUI

struct EmptyBibListView: View {
    @Environment(ThemeManager.self) private var themeManager: ThemeManager

    @State private var createSheetOpen = false
    @Binding var editingBibEntry: BibEntryModel?
    @Environment(NoteModel.self) private var editingNote: NoteModel?
    @Environment(\.modelContext) var modelContext

    let container: BibtexEditorWebviewContainer

    var body: some View {
        NavigationStack {
            VStack(spacing: 16) {
                ZStack {
                    themeManager.theme.primary
                        .frame(width: 64, height: 64)
                        .clipShape(Circle())
                    Image(systemName: "magnifyingglass")
                        .imageScale(.large)
                        .foregroundStyle(themeManager.theme.primary_foreground)
                }
                Text("No bibliography entries found")
                    .font(.title2)
                NavigationLink(
                    destination: CreateBibEntrySheetView(
                        editingBibEntry: $editingBibEntry,
                        container: container
                    ),
                    label: {
                        Text("Create")
                    }
                ).onTapGesture {
                    editingBibEntry = nil
                }
                if let _editingNote = editingNote {
                    if NoteModel.count(modelContext: modelContext) > 0 {
                        @Bindable var editingNoteBinding = _editingNote
                        NavigationLink(
                            destination: AssociateNoteWithBibEntryView(
                                editingNote: editingNoteBinding
                            ),
                            label: {
                                Text("Search")
                            }
                        )
                    }
                }
            }
        }
    }
}

#Preview {
    EmptyBibListView(
        editingBibEntry: .constant(nil),
        container: BibtexEditorWebviewContainer()
    )
}
