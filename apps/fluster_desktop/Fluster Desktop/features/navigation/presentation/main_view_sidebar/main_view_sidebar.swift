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
  let items: [SidebarItem] = mainSidebarItems
  var body: some View {
    CollapsableSidebarSection(
      open: $noteSidebarSectionOpen,
      items: noteSideBarItems,
      title: "Note"
    )
    CollapsableSidebarSection(
      open: $flusterSidebarSectionOpen,
      items: items,
      title: "Fluster"
    )
    .listStyle(.sidebar)
    .navigationTitle("Fluster")
    .navigationSplitViewColumnWidth(min: 180, ideal: 200)
    Spacer()
  }
}

#Preview {
  MainViewSidebar()
}
