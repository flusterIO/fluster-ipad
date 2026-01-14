//
//  main_view_sidebar.swift
//  fluster_desktop
//
//  Created by Andrew on 1/14/26.
//

import SwiftUI

struct MainViewSidebar: View {
  @Binding var mainView: MainViewKey
  let items: [SidebarItem] = mainSidebarItems
  var body: some View {
    List(selection: $mainView) {
      ForEach(items) { item in
        if item.children == nil {
          Label(
            title: {
              Text(item.label)
            },
            icon: {
              Image(systemName: item.icon)
            })
        } else {
          Section(item.label) {
            List(item.children!) { child in
              Label(
                title: {
                  Text(child.label)
                },
                icon: {
                  Image(systemName: child.icon)
                })
            }
          }
        }
      }
    }
    .listStyle(.sidebar)
    .navigationTitle("Fluster")
    .navigationSplitViewColumnWidth(min: 180, ideal: 200)
  }
}

#Preview {
  MainViewSidebar(mainView: .constant(.dashboard))
}
