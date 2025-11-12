//
//  dashboard_view.swift
//  Fluster
//
//  Created by Andrew on 10/29/25.
//

import PencilKit
import SwiftData
import SwiftUI

enum MainViewTab {
    case paper, markdown, bib, notes
}


func getDrawing(data: Data?) -> PKDrawing {
    if data == nil {
        return PKDrawing()
    }
    
    do {
        let drawing = try PKDrawing(data: data!)
        return drawing
    } catch {
        return PKDrawing()
    }
}

struct MainView: View {
    @State private var toolbar = PKToolPicker()
    @State private var canvasView = PKCanvasView()
    @Environment(\.colorScheme) var colorScheme: ColorScheme
    @State private var themeManager = ThemeManager(
        initialTheme: getTheme(themeName: .Fluster, darkMode: true)
    )
    @State private var selectedTab = MainViewTab.paper
    @State private var editingNoteId: String?
    @State private var editingNote: NoteModel?

    var body: some View {
        TabView(selection: $selectedTab) {
            Tab(
                "Paper",
                systemImage: "pencil.circle.fill",
                value: MainViewTab.paper
            ) {
                if editingNote != nil {
                    PaperView(
                        toolbar: $toolbar,
                        canvasView: $canvasView,
                        drawing: editingNote != nil ? getDrawing(data: editingNote!.drawing) : PKDrawing()
                    )
                } else {
                     SelectNoteToContinueView()
                }
            }
            Tab(
                "Markdown",
                systemImage: "book.closed.circle.fill",
                value: MainViewTab.markdown
            ) {
                EditorSplitView(editingNote: $editingNote)
            }
            Tab(
                "Bibliography",
                systemImage: "books.vertical.circle.fill",
                value: MainViewTab.bib
            ) {
                BibliographyPageView()
            }
            Tab(
                "Search",
                systemImage: "magnifyingglass.circle.fill",
                value: MainViewTab.notes
            ) {
                SearchPageView(editingNote: $editingNote)
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
        .onAppear {
            handleColorSchemeChange(newScheme: colorScheme)
        }
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
