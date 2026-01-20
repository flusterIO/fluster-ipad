//
//  main_view_switch.swift
//  fluster_desktop
//
//  Created by Andrew on 1/14/26.
//

import SwiftUI

struct MainViewSwitch: View {
  @Environment(AppState.self) private var appState: AppState
  var body: some View {
    switch appState.mainView {
      case .createNote:
        CreateNotePage()
            .navigationTitle("Create Note")
      case .noteEditingPage:
        EditMdxPageView()
      case .dashboard:
        ModularDashboardView()
            .navigationTitle("")
      case .globalBibliography:
        GlobalBibliographyPageView()
            .navigationTitle("Bibliography")
    case .noteViewMdx:
        ViewEditingNoteMdxPage()
      case .bookmarks:
        BookmarksPageView()
            .navigationTitle("Bookmarks")
      case .search:
        GlobalSearchPage()
            .navigationTitle("Search")
      case .settings:
        MainSettingsPageView()
    }
  }
}

#Preview {
  MainViewSwitch()
}
