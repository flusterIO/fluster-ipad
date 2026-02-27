//
//  bibliography_page_view.swift
//  Fluster
//
//  Created by Andrew on 10/29/25.
//

import FlusterData
import FlusterSwift
import SwiftData
import SwiftUI

struct BibliographyPageInternalView: View {
  @Query var bibEntries: [BibEntryModel]
  @State var associateNoteModalPresented: Bool = false
  @Binding var editingBibEntry: BibEntryModel?
  @Binding var editingNote: NoteModel?
  @State private var searchQuery: String = ""
  let bibtexEditorContainer: BibtexEditorWebviewContainer
  var filteredEntries: [BibEntryModel] {
    if searchQuery.isEmpty {
      return editingNote?.citations ?? []
    } else {
      do {
        let res = try (editingNote?.citations ?? []).filter(
          #Predicate<BibEntryModel> { entry in
            entry._data.localizedStandardContains(searchQuery)
          })
        return res
      } catch {
        print("Error: \(error.localizedDescription)")
        return editingNote?.citations ?? []
      }
    }
  }

  var body: some View {
    if let _editingNote = editingNote {
      ZStack(alignment: .bottomTrailing) {
        if _editingNote.citations.isEmpty {
          EmptyBibListView(
            editingBibEntry: $editingBibEntry,
            container: bibtexEditorContainer,
            ignoreBibEntryOnCreate: false,
            associateNoteModalPresented:
              $associateNoteModalPresented,
          )
          .navigationTitle("Note Bibliography")
        } else {
          BibListView(
            items: filteredEntries,
            editingBibEntry: $editingBibEntry,
            editingNote: $editingNote,
            editorContainer: bibtexEditorContainer
          )
          .searchable(text: $searchQuery, placement: .automatic, prompt: "Search")
          .toolbar {
            ToolbarItem(
              content: {
                NavigationLink(
                  destination: {
                    CreateBibEntrySheetView(
                      editingBibEntry: .constant(nil),
                      ignoreEditingNote: false,
                      container: bibtexEditorContainer
                    )
                  },
                  label: {
                    Label(
                      "Create",
                      systemImage: "plus"
                    )
                  }
                )
              })
            ToolbarItem(
              content: {
                Button(
                  action: {
                    associateNoteModalPresented = true
                  },
                  label: {
                    Label(
                      "Link",
                      systemImage: "link"
                    )
                  }
                )
              })
          }
          .navigationTitle("Note Bibliography")
        }
      }
      .fullScreenCover(
        isPresented: $associateNoteModalPresented,
        content: {
          if let editingNoteBinding = Binding($editingNote) {
            NavigationStack {
              AssociateNoteWithBibEntryView(
                editingNote: editingNoteBinding,
                open: $associateNoteModalPresented
              )
            }
          } else {
            Color.clear.onAppear {
              associateNoteModalPresented = false
            }
          }
        }
      )
    } else {
      SelectNoteToContinueView()
    }
  }
}

struct BibliographyPageView: View {
  @State private var editingBibEntry: BibEntryModel? = nil
  @AppStorage(AppStorageKeys.editorThemeDark.rawValue) private
    var editorThemeDark: CodeSyntaxTheme = .dracula
  @AppStorage(AppStorageKeys.editorThemeLight.rawValue) private
    var editorThemeLight: CodeSyntaxTheme = .githubLight
  @AppStorage(AppStorageKeys.editorKeymap.rawValue) private var editorKeymap: EditorKeymap = .base
  @AppStorage(AppStorageKeys.theme.rawValue) private var theme: WebViewTheme =
    .fluster
  @Environment(\.colorScheme) var colorScheme
  @StateObject private var bibtexEditorContainer =
    BibtexEditorWebviewContainer(bounce: true, scrollEnabled: true, onLoad: nil)
  @Binding var editingNote: NoteModel?
  var body: some View {
    BibliographyPageInternalView(
      editingBibEntry: $editingBibEntry,
      editingNote: $editingNote,
      bibtexEditorContainer: bibtexEditorContainer
    )
    .onChange(
      of: editingBibEntry,
      {
        if editingBibEntry != nil {
          bibtexEditorContainer.setInitialContent(
            entryBody: editingBibEntry!.data
          )
        }
      }
    )
    .onChange(
      of: editorThemeDark,
      {
        bibtexEditorContainer.emitEditorThemeEvent(
          theme: colorScheme == .dark
            ? editorThemeDark : editorThemeLight
        )
      }
    )
    .onChange(
      of: editorThemeLight,
      {
        bibtexEditorContainer.emitEditorThemeEvent(
          theme: colorScheme == .dark
            ? editorThemeDark : editorThemeLight
        )
      }
    )
    .onChange(
      of: editorKeymap,
      {
        bibtexEditorContainer.setEditorKeymap(
          editorKeymap: editorKeymap
        )
      }
    )
    .onChange(
      of: theme,
      {
        bibtexEditorContainer.setWebviewTheme(theme: theme)
      }
    )
  }
}

#Preview {
  BibliographyPageView(editingNote: .constant(nil))
}
