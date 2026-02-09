//
//  main_view_switch.swift
//  fluster_desktop
//
//  Created by Andrew on 1/14/26.
//

import FlusterData
import SwiftUI
import SwiftData
import WebKit

struct MainViewSwitch: View {
  @EnvironmentObject private var appState: AppState
  @Environment(\.modelContext) private var modelContext
  var editingNote: NoteModel? {
    guard let id = appState.editingNoteId else { return nil }
    return modelContext.model(for: id) as? NoteModel
  }

  @State private var editorWebview: WKWebView = WKWebView(
    frame: .infinite, configuration: getWebViewConfig()
  )

  var body: some View {
    switch appState.mainView {
      case .dashboard:
        ModularDashboardView()
          .navigationTitle("")
      case .createNote:
        CreateNotePage()
          .navigationTitle("Create Note")
      case .paper:
        ViewPaperPageView()
          .navigationTitle("Paper")
      case .noteEditingPage:
        EditMdxPageView(webview: $editorWebview)
          .navigationTitle("Editor")
          .onAppear {
            markEditingNoteRead()
          }
      case .noteViewMdx:
        ViewEditingNoteMdxPage()
          .navigationTitle("Markdown")
          .onAppear {
            markEditingNoteRead()
          }
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
    if let _editingNote = editingNote {
      _editingNote.setLastRead()
    }
  }
}

#Preview {
  MainViewSwitch()
}
