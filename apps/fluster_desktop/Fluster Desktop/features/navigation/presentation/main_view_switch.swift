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
        ViewPaperPageView(editingNoteId: appState.editingNoteId)
      case .noteEditingPage:
        MdxEditorWebview(editingNoteId: appState.editingNoteId, webView: $editorWebview)
          .navigationTitle("Editor")
      case .noteViewMdx:
        MdxContentWebview(editingNoteId: appState.editingNoteId)
          .navigationTitle("Markdown")
      case .globalBibliography:
        GlobalBibliographyPageView()
          .navigationTitle("Shared Bibliography")
      case .editingNoteBibliography:
        EditingNoteBibliographyPageView(editingNoteId: appState.editingNoteId)
          .navigationTitle("Note Bibliography")
      case .editingNoteDetails:
        NoteDetailWebview(editingNoteId: appState.editingNoteId)
          .navigationTitle("Note Details")
      case .globalDictionary:
        DictionaryPageView()
          .navigationTitle("Dictionary")
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
}

#Preview {
  MainViewSwitch()
}
