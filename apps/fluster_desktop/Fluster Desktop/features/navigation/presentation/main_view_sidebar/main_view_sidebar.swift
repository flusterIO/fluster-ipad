//
//  main_view_sidebar.swift
//  fluster_desktop
//
//  Created by Andrew on 1/14/26.
//

import SwiftUI

struct MainViewSidebar: View {
  @AppStorage(DesktopAppStorageKeys.flusterSidebarSectionOpen.rawValue) private
    var flusterSidebarSectionOpen = false
  @AppStorage(DesktopAppStorageKeys.noteSidebarSectionOpen.rawValue) private
    var noteSidebarSectionOpen = false
  var body: some View {
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
            open: $flusterSidebarSectionOpen,
            items: globalSearchSidebarItems,
            title: "Global Search"
          )
      }
    .listStyle(.sidebar)
    .navigationTitle("Fluster")
    .navigationSplitViewColumnWidth(min: 180, ideal: 200)
    Spacer()
  }
}

#Preview {
  MainViewSidebar()
}
