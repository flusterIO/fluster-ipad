//
//  empty_bib_list_view.swift
//  Fluster
//
//  Created by Andrew on 11/3/25.
//

import FlusterSwift
import SwiftData
import SwiftUI

struct EmptyBibListView: View {
  @Environment(ThemeManager.self) private var themeManager: ThemeManager
  @State private var createSheetOpen = false
  @Environment(NoteModel.self) private var editingNote: NoteModel?
  @Environment(\.modelContext) var modelContext

  @Binding var editingBibEntry: BibEntryModel?

  let container: BibtexEditorWebviewContainer
  /// If true, the nested CreateBibEntrySheetView will automatically link bibliography entries to the note being edited.
  var ignoreBibEntryOnCreate: Bool = true
  @Binding var associateNoteModalPresented: Bool

  init(
    editingBibEntry: Binding<BibEntryModel?>,
    container: BibtexEditorWebviewContainer,
    ignoreBibEntryOnCreate: Bool = false,
    associateNoteModalPresented: Binding<Bool>
  ) {
    self._editingBibEntry = editingBibEntry
    self.container = container
    self.ignoreBibEntryOnCreate = ignoreBibEntryOnCreate
    self._associateNoteModalPresented = associateNoteModalPresented
  }

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
            ignoreEditingNote: ignoreBibEntryOnCreate,
            container: container
          ),
          label: {
            Label("Create", systemImage: "plus")
          }
        ).onTapGesture {
          editingBibEntry = nil
        }
        if editingNote != nil {
          if NoteModel.count(modelContext: modelContext) > 0 {
            Button(
              action: {
                associateNoteModalPresented = true
              },
              label: {
                Label("Search", systemImage: "magnifyingglass")
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
    container: BibtexEditorWebviewContainer(
      bounce: true,
      scrollEnabled: true
    ),
    associateNoteModalPresented: .constant(false)
  )
}
