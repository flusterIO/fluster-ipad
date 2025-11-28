//
//  bibliography_page_view.swift
//  Fluster
//
//  Created by Andrew on 10/29/25.
//

import SwiftData
import SwiftUI
import FlusterSwift

struct BibliographyPageInternalView: View {
    @Binding var editing: BibEntryModel?
    @Query var bibEntries: [BibEntryModel]
    @Environment(NoteModel.self) private var editingNote: NoteModel?
    let bibtexEditorContainer: BibtexEditorWebviewContainer
    var body: some View {
        if let _editingNote = editingNote {
            ZStack(alignment: .bottomTrailing) {
                if _editingNote.citations.isEmpty {
                    EmptyBibListView(
                        editingBibEntry: $editing,
                        container: bibtexEditorContainer
                    )
                    .navigationTitle("Note Bibliography")
                } else {
                    BibListView(
                        items: _editingNote.citations,
                        editing: $editing,
                        editorContainer: bibtexEditorContainer
                    )
                    .navigationTitle("Note Bibliography")
                    BibliographyPageFloatingButtonView(
                        editing: $editing,
                        bibEditorContainer: bibtexEditorContainer
                    )
                }
            }
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
