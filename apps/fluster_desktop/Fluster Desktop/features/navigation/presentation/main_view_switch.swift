//
//  main_view_switch.swift
//  fluster_desktop
//
//  Created by Andrew on 1/14/26.
//

import SwiftUI
import FlusterData

struct MainViewSwitch: View {
  @Environment(AppState.self) private var appState: AppState
  var body: some View {
    switch appState.mainView {
      case .dashboard:
        ModularDashboardView()
          .navigationTitle("")
      case .createNote:
        CreateNotePage()
          .navigationTitle("Create Note")
      case .noteEditingPage:
        EditMdxPageView()
          .onAppear {
            markEditingNoteRead()
          }
      case .noteViewMdx:
        ViewEditingNoteMdxPage()
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
    if let editingNote = appState.editingNote {
      editingNote.setLastRead()
    }
  }
}

#Preview {
  MainViewSwitch()
}
