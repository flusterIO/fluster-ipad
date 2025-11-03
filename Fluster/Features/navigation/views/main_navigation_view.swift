//
//  dashboard_view.swift
//  Fluster
//
//  Created by Andrew on 10/29/25.
//

import PencilKit
import SwiftUI

enum MainViewTab {
    case paper, markdown, bib, notes
}

struct MainView: View {
    @State private var toolbar = PKToolPicker()
    @State private var canvasView = PKCanvasView()
    @State private var themeManager = ThemeManager(initialTheme: FlusterDark())
    @State private var selectedTab = MainViewTab.paper
    @Environment(\.colorScheme) var colorScheme: ColorScheme
    var body: some View {
        TabView(selection: $selectedTab) {
            Tab(
                "Paper",
                systemImage: "pencil.circle.fill",
                value: MainViewTab.paper
            ) {
                PaperView(toolbar: $toolbar, canvasView: $canvasView)
            }
            Tab(
                "Markdown",
                systemImage: "book.closed.circle.fill",
                value: MainViewTab.markdown
            ) {
                EditorSplitView()
                    .background(themeManager.theme.background)
            }
            Tab(
                "Bibliography",
                systemImage: "books.vertical.circle.fill",
                value: MainViewTab.bib
            ) {
                BibliographyPageView()
            }
            Tab(
                "Notes",
                systemImage: "magnifyingglass.circle.fill",
                value: MainViewTab.notes
            ) {
                SearchPageView()
                    .ignoresSafeArea()
            }
        }
        .onChange(
            of: selectedTab,
            {
                if selectedTab == .paper {
                    toolbar.setVisible(true, forFirstResponder: canvasView)
                    toolbar.addObserver(canvasView)
                    canvasView.becomeFirstResponder()
                }
            }
        )
        .environment(themeManager)
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
}
