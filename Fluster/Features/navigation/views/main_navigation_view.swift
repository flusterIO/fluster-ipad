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
    @Environment(\.colorScheme) var colorScheme: ColorScheme
    @State private var themeManager = ThemeManager(initialTheme: getTheme(themeName: .Fluster, darkMode: true))
    @State private var selectedTab = MainViewTab.paper
    @State private var editingNoteId: String?
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
                SearchPageView(editingNoteId: $editingNoteId)
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
        .onChange(
            of: colorScheme,
            {
                handleColorSchemeChange(newScheme: colorScheme)
            }
        )
        .environment(themeManager)
    }
    func handleColorSchemeChange(newScheme: ColorScheme) {
        self.themeManager = ThemeManager(
            initialTheme: getTheme(
                themeName: .Fluster,
                darkMode: colorScheme == .dark
            )
        )
    }
}

#Preview {
    MainView()
        .environment(ThemeManager(initialTheme: FlusterDark()))
}
