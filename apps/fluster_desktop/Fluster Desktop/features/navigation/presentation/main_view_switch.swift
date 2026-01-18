//
//  main_view_switch.swift
//  fluster_desktop
//
//  Created by Andrew on 1/14/26.
//

import SwiftUI

struct MainViewSwitch: View {
  @Binding var mainView: MainViewKey
  var body: some View {
    switch mainView {
      case .createNote:
        CreateNotePage()
      case .noteEditingPage:
        EditMdxPageView()
      case .dashboard:
        ModularDashboardView()
      case .search:
        SearchPageView()
      case .settings:
        MainSettingsPageView()
    }
  }
}

#Preview {
  MainViewSwitch(mainView: .constant(.dashboard))
}
