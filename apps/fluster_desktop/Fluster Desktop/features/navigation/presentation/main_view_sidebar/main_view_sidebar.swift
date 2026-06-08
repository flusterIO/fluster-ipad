//
//  main_view_sidebar.swift
//  fluster_desktop
//
//  Created by Andrew on 1/14/26.
//

import SwiftUI
import FlusterData

struct MainViewSidebar: View {
  @AppStorage(AppStorageKeys.flusterSidebarSectionOpen.rawValue) private
    var flusterSidebarSectionOpen = true
  @AppStorage(AppStorageKeys.noteSidebarSectionOpen.rawValue) private
    var noteSidebarSectionOpen = true
    @AppStorage(AppStorageKeys.globalSectionOpen.rawValue) private
      var globalSectionOpen = true
  var body: some View {
    ScrollView {
      Group {
        CollapsableSidebarSection(
          open: $noteSidebarSectionOpen,
          items: noteSideBarItems,
          title: "Note"
        )
        CollapsableSidebarSection(
          open: $flusterSidebarSectionOpen,
          items: mainSidebarItems,
          title: "Fluster"
        )
        CollapsableSidebarSection(
          open: $globalSectionOpen,
          items: globalSearchSidebarItems,
          title: "Global Search"
        )
      }
    }
    .scrollIndicators(.never)
    .listStyle(.sidebar)
    .navigationTitle("Fluster")
    .navigationSplitViewColumnWidth(min: 180, ideal: 200)
    Spacer()
  }
}

#Preview {
  MainViewSidebar()
}
