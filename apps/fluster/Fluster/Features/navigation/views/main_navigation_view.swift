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
    case paper, markdown, bib, notes, settings
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

func getInitialTheme() -> WebViewTheme {
    if let storedString = UserDefaults.standard.string(
        forKey: AppStorageKeys.theme.rawValue
    ) {
        if let theme = WebViewTheme(rawValue: storedString) {
            return theme
        }
    }
    return .fluster
}

struct MainView: View {
    @State private var toolbar = PKToolPicker()
    @State private var canvasView = PKCanvasView()
    @AppStorage(AppStorageKeys.theme.rawValue) private var theme: WebViewTheme =
        .fluster
    @Environment(\.colorScheme) var colorScheme: ColorScheme
    @AppStorage(AppStorageKeys.colorScheme.rawValue) private
        var colorSchemeSelection: ColorSchemeSelection = .dark
    @AppStorage(AppStorageKeys.editorTheme.rawValue) private var editorTheme:
        CodeEditorTheme = .materialDark
    @State private var themeManager = ThemeManager(
        initialTheme: getTheme(themeName: getInitialTheme(), darkMode: true)
    )
    @State private var selectedTab = MainViewTab.paper
    @State private var editingNoteId: String?
    @State private var editingNote: NoteModel?
    @State private var findInNotePresented: Bool = false
    

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
                        drawing: editingNote != nil
                            ? getDrawing(data: editingNote!.drawing)
                            : PKDrawing()
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
                if editingNote != nil {
                    ResponsiveEditorWebView(
                        url:
                            Bundle.main.url(
                                forResource: "index",
                                withExtension: "html",
                                subdirectory: "dist"
                            )!,
                        theme: $theme,
                        editorTheme: $editorTheme,
                        editingNote: $editingNote
                    )
                    .findNavigator(isPresented: $findInNotePresented)
                    .ignoresSafeArea(edges: .bottom)
                    .toolbar {
                        ToolbarItemGroup {
                            Button("Find", systemImage: "magnifyingglass") {
                                findInNotePresented.toggle()
                            }
                        }
                    }
                } else {
                    SelectNoteToContinueView()
                }
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
                value: MainViewTab.notes,
                role: .search
            ) {
                SearchPageView(editingNote: $editingNote)
                    .ignoresSafeArea()
            }
            Tab(
                "Settings",
                systemImage: "gearshape.fill",
                value: MainViewTab.settings
            ) {
                SettingsPageView(
                    theme: $theme,
                    editorTheme: $editorTheme,
                    colorSchemeSelection: $colorSchemeSelection
                )
            }
            .tabPlacement(.pinned)
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
        .onChange(
            of: theme,
            {
                handleThemeChange(newTheme: theme)
            }
        )
        .onChange(
            of: colorSchemeSelection,
            {
                handleColorSchemeSelectionChange()
            }
        )
        .onAppear {
            handleColorSchemeChange(newScheme: colorScheme)
            handleThemeChange(newTheme: theme)
            handleColorSchemeSelectionChange()
        }
        .background(themeManager.theme.background)
        .presentationBackground(themeManager.theme.background)
        .accentColor(themeManager.theme.primary)
        .preferredColorScheme(
            getColorScheme(
                selected: colorSchemeSelection,
                systemScheme: colorScheme
            )
        )
        .environment(themeManager)
    }
    func handleColorSchemeSelectionChange() {
        handleColorSchemeChange(
            newScheme: getColorScheme(
                selected: colorSchemeSelection,
                systemScheme: colorScheme
            )
        )
    }
    func handleThemeChange(newTheme: WebViewTheme) {
        self.themeManager = ThemeManager(
            initialTheme: getTheme(
                themeName: newTheme,
                darkMode: colorScheme == .dark
            )
        )
    }
    func handleColorSchemeChange(newScheme: ColorScheme) {
        self.themeManager = ThemeManager(
            initialTheme: getTheme(
                themeName: theme,
                darkMode: newScheme == .dark
            )
        )
    }
}

#Preview {
    MainView()
        .environment(ThemeManager(initialTheme: FlusterDark()))
}
