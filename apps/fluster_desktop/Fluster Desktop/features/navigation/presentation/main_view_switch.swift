//
//  main_view_switch.swift
//  fluster_desktop
//
//  Created by Andrew on 1/14/26.
//

import FlusterData
import SwiftData
import SwiftUI
import WebKit

struct MainViewSwitch: View {
  @EnvironmentObject private var appState: AppState
  @Environment(\.modelContext) private var modelContext
  public var editingNoteId: String?
  @Query private var notes: [NoteModel]

  var editingNote: NoteModel? {
    notes.isEmpty ? nil : notes.first
  }

  @State private var editorWebview: WKWebView = WKWebView(
    frame: .infinite, configuration: getWebViewConfig()
  )

  public init(editingNoteId: String?) {
    self.editingNoteId = editingNoteId
    if let _id = editingNoteId {
      let predicate = #Predicate<NoteModel> { $0.id == _id }
      _notes = Query(filter: predicate)
    } else {
      _notes = Query(
        filter: #Predicate<NoteModel> { note in
          false
        })
    }
  }

  var body: some View {
    switch appState.mainView {
      case .dashboard:
        ModularDashboardView()
          .navigationTitle("")
      case .createNote:
        CreateNotePage()
          .navigationTitle("Create Note")
      case .paper:
        ViewPaperPageView(editingNoteId: appState.editingNoteId)
          .navigationTitle("Paper")
      case .noteEditingPage:
        MdxEditorWebview(editingNoteId: appState.editingNoteId, webView: $editorWebview)
          .navigationTitle("Editor")
      case .noteViewMdx:
        MdxContentWebview(editingNoteId: appState.editingNoteId)
          .navigationTitle("Markdown")
      case .globalBibliography:
        GlobalBibliographyPageView()
          .navigationTitle("Bibliography")
      case .bookmarks:
        BookmarksPageView()
          .navigationTitle("Bookmarks")
      case .search:
        GlobalSearchPage()
          .navigationTitle("Search")
      case .settings:
        MainSettingsPageView()
          .navigationTitle("")
    }
  }

  func markEditingNoteRead() {
    if let _editingNote = editingNote, editingNoteIsValid(note: _editingNote, appState: appState) {
      _editingNote.setLastRead()
    }
  }
}

#Preview {
  MainViewSwitch(editingNoteId: nil)
}
