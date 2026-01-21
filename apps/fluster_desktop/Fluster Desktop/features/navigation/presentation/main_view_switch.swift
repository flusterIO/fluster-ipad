//
//  main_view_switch.swift
//  fluster_desktop
//
//  Created by Andrew on 1/14/26.
//

import FlusterData
import SwiftUI

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
      case .paper:
        ViewPaperPageView()
            .navigationTitle("Paper")
      case .noteEditingPage:
        EditMdxPageView()
          .navigationTitle("Mdx Editor")
          .onAppear {
            markEditingNoteRead()
          }
      case .noteViewMdx:
        ViewEditingNoteMdxPage()
          .navigationTitle("View Markdown")
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
