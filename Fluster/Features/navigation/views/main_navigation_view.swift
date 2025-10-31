//
//  dashboard_view.swift
//  Fluster
//
//  Created by Andrew on 10/29/25.
//

import SwiftUI

struct MainView: View {
    @State private var sidebarOpen: Bool = false
    @State private var visibility: NavigationSplitViewVisibility = .detailOnly
    @Environment(Coordinator<MainCoordinatorPages>.self) private var coordinator
    @Environment(ThemeManager.self) private var themeManager: ThemeManager
    @Environment(\.colorScheme) var colorScheme: ColorScheme
    var body: some View {
        NavigationStack {
            ZStack {
                EditorSplitView()
                    .background(themeManager.theme.background).slideInView(isActive: $sidebarOpen, content: {
                        SideMenuView(open: $sidebarOpen)
                    })
            }
            .toolbar(sidebarOpen ? .hidden : .visible, for: .navigationBar)
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(
                    placement: .topBarLeading,
                    content: {
                        Button(
                            action: {
                                sidebarOpen.toggle()
                            },
                            label: {
                                Image(systemName: "line.3.horizontal")
                                    .shadow(radius: 0)
                            }
                        )
                    }
                )
            }
        }
    }
    func handleColorSchemeChange(newScheme: ColorScheme) {
        self.themeManager.theme = getTheme(
            themeName: .Fluster,
            darkMode: colorScheme == .dark
        )
    }
}

#Preview {
    MainView()
        .environment(ThemeManager(initialTheme: FlusterDark()))
        .environment(Coordinator<MainCoordinatorPages>())
}
