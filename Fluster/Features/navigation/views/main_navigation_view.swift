//
//  dashboard_view.swift
//  Fluster
//
//  Created by Andrew on 10/29/25.
//

import SwiftUI

struct MainView: View {
    @State private var visibility: NavigationSplitViewVisibility = .detailOnly
    @Environment(Coordinator<MainCoordinatorPages>.self) private var coordinator
    var body: some View {
        NavigationSplitView(columnVisibility: $visibility) {
            List {
                ForEach(navigationItems) { navItem in
                    HStack{
                        Image(systemName: navItem.icon)
                        Text(navItem.label)
                    }
                    .onTapGesture {
                        if navItem.path == MainCoordinatorPages.root {
                            coordinator.popToRoot()
                        } else {
                            coordinator.push(navItem.path)
                        }
                    }
                }
            }
        } detail: {
            EditorSplitView()
        }
    }
}

#Preview {
    MainView()
}
