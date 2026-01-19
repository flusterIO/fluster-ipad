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
      case .noteEditingPage:
        EditMdxPageView()
      case .dashboard:
        ModularDashboardView()
      case .globalBibliography:
        GlobalBibliographyPageView()
      case .bookmarks:
        BookmarksPageView()
      case .search:
        SearchPageView()
      case .settings:
        MainSettingsPageView()
    }
  }
}

#Preview {
  MainViewSwitch()
}
