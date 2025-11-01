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
        TabView {
            Tab("Markdown", systemImage: "book.closed.circle.fill") {
                ZStack {
                    EditorSplitView()
                        .background(themeManager.theme.background).slideInView(
                            isActive: $sidebarOpen,
                            content: {
                                SideMenuView(open: $sidebarOpen)
                            }
                        )
                }
            }
            Tab("Paper", systemImage: "pencil.circle.fill") {
                ZStack {
                    PaperView()
                }
            }
            Tab("Bibliography", systemImage: "books.vertical.circle.fill") {
                ZStack {
                    BibliographyPageView()
                }
            }
            Tab("Search", systemImage: "magnifyingglass.circle.fill") {
                ZStack {
                    SearchResultsPageView()
                }
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
