//
//  bibliography_page_view.swift
//  Fluster
//
//  Created by Andrew on 10/29/25.
//

import FlusterSwift
import SwiftData
import SwiftUI

struct BibliographyPageInternalView: View {
    @Query var bibEntries: [BibEntryModel]
    @Environment(NoteModel.self) private var editingNote: NoteModel?
    @State var associateNoteModalPresented: Bool = false
    @Binding var editing: BibEntryModel?
    let bibtexEditorContainer: BibtexEditorWebviewContainer

    var body: some View {
        if let _editingNote = editingNote {
            ZStack(alignment: .bottomTrailing) {
                if _editingNote.citations.isEmpty {
                    EmptyBibListView(
                        editingBibEntry: $editing,
                        container: bibtexEditorContainer,
                        associateNoteModalPresented:
                            $associateNoteModalPresented
                    )
                    .navigationTitle("Note Bibliography")
                } else {
                    BibListView(
                        items: _editingNote.citations,
                        editing: $editing,
                        editorContainer: bibtexEditorContainer
                    )
                    .toolbar {
                        Button(action: {
                            associateNoteModalPresented = true
                        }, label: {
                            Label("Search", systemImage: "magnifyingglass")
                        })
                    }
                    .navigationTitle("Note Bibliography")
                }
            }
            .fullScreenCover(
                isPresented: $associateNoteModalPresented,
                content: {
                    if let editingNoteExists = editingNote {
                        @Bindable var editingNoteBinding = editingNoteExists
                        NavigationStack {
                            AssociateNoteWithBibEntryView(
                                editingNote: editingNoteBinding,
                                open: $associateNoteModalPresented
                            )
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
    @State private var editing: BibEntryModel? = nil
    @AppStorage(AppStorageKeys.editorThemeDark.rawValue) private
        var editorThemeDark: CodeEditorTheme = .dracula
    @AppStorage(AppStorageKeys.editorThemeLight.rawValue) private
        var editorThemeLight: CodeEditorTheme = .githubLight
    @AppStorage(AppStorageKeys.editorKeymap.rawValue) private var editorKeymap:
        EditorKeymap = .base
    @AppStorage(AppStorageKeys.theme.rawValue) private var theme: WebViewTheme =
        .fluster
    @Environment(\.colorScheme) var colorScheme
    @StateObject private var bibtexEditorContainer =
        BibtexEditorWebviewContainer()
    var body: some View {
        BibliographyPageInternalView(
            editing: $editing,
            bibtexEditorContainer: bibtexEditorContainer
        )
        .onChange(
            of: editing,
            {
                if editing != nil {
                    bibtexEditorContainer.setInitialContent(
                        entryBody: editing!.data
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
    BibliographyPageView()
}
