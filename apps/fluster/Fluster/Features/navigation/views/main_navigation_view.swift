//
//  dashboard_view.swift
//  Fluster
//
//  Created by Andrew on 10/29/25.
//

import PencilKit
import SwiftData
import SwiftUI

enum MainViewTab: String {
    case paper, markdown, bib, notes, settings, createNote, searchNotes,
        searchByBib
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
    @AppStorage(AppStorageKeys.theme.rawValue) private var theme: WebViewTheme =
        .fluster
    @AppStorage(AppStorageKeys.webviewFontSize.rawValue) private
        var webviewFontSize: WebviewFontSize = .base
    @Environment(\.colorScheme) var colorScheme: ColorScheme
    @AppStorage(AppStorageKeys.colorScheme.rawValue) private
        var colorSchemeSelection: ColorSchemeSelection = .dark
    @AppStorage(AppStorageKeys.editorThemeDark.rawValue) private
        var editorThemeDark: CodeEditorTheme = .dracula
    @AppStorage(AppStorageKeys.editorThemeLight.rawValue) private
        var editorThemeLight: CodeEditorTheme = .githubLight
    @AppStorage(AppStorageKeys.editorKeymap.rawValue) private var editorKeymap:
        EditorKeymap = .base
    @AppStorage(AppStorageKeys.tabviewCustomization.rawValue) private
        var tabviewCustomization: TabViewCustomization

    @State private var themeManager = ThemeManager(
        initialTheme: getTheme(themeName: getInitialTheme(), darkMode: true)
    )
    @State private var selectedTab = MainViewTab.paper
    @State private var editingNoteId: String?
    @State private var editingNote: NoteModel?
    @State private var findInNotePresented: Bool = false
    @StateObject private var editorContainer = EditorWebViewContainer()

    var body: some View {
        TabView(selection: $selectedTab) {
            Tab(
                "Paper",
                systemImage: "pencil.circle.fill",
                value: MainViewTab.paper
            ) {
                if let unwrappedEditingNote = Binding($editingNote) {
                    PaperView(
                        toolbar: $toolbar,
                        drawingData: unwrappedEditingNote.drawing,
                        activeTab: $selectedTab
                    )
                    .backgroundExtensionEffect()
                } else {
                    SelectNoteToContinueView()
                }
            }
            .customizationID(MainViewTab.paper.rawValue)
            .defaultVisibility(.visible, for: .tabBar)
            Tab(
                "Markdown",
                systemImage: "book.closed.circle.fill",
                value: MainViewTab.markdown
            ) {
                if editingNote != nil {
                    WebViewWrapper(
                        url:
                            Bundle.main.url(
                                forResource: "index",
                                withExtension: "html",
                                subdirectory: "dist"
                            )!,
                        theme: $theme,
                        editorThemeDark: $editorThemeDark,
                        editorThemeLight: $editorThemeLight,
                        editingNote: $editingNote,
                        editorKeymap: $editorKeymap,
                        container: editorContainer,
                    )
                    .onChange(
                        of: editingNote,
                        {
                            if let note = editingNote {
                                editorContainer.setInitialContent(
                                    note: note
                                )
                                editorContainer.resetScrollPosition()
                                note.last_read = .now
                            }
                        }
                    )
                    .onChange(
                        of: editorThemeDark,
                        {
                            editorContainer.emitEditorThemeEvent(
                                theme: colorScheme == .dark
                                    ? editorThemeDark : editorThemeLight
                            )
                            editorContainer.setEditorDarkTheme(
                                theme: editorThemeDark
                            )
                        }
                    )
                    .onChange(
                        of: editorThemeLight,
                        {
                            editorContainer.emitEditorThemeEvent(
                                theme: colorScheme == .dark
                                    ? editorThemeDark : editorThemeLight
                            )
                            editorContainer.setEditorLightTheme(
                                theme: editorThemeLight
                            )
                        }
                    )
                    .onChange(
                        of: editorKeymap,
                        {
                            editorContainer.setEditorKeymap(
                                editorKeymap: editorKeymap
                            )
                        }
                    )
                } else {
                    SelectNoteToContinueView()
                }
            }
            .customizationID(MainViewTab.markdown.rawValue)
            .defaultVisibility(.visible, for: .tabBar)
            Tab(
                "Bibliography",
                systemImage: "books.vertical.circle.fill",
                value: MainViewTab.bib
            ) {
                BibliographyPageView()
                    .ignoresSafeArea(edges: .horizontal)
            }
            .customizationID(MainViewTab.bib.rawValue)
            .defaultVisibility(.visible, for: .tabBar)
            Tab(
                "Search",
                systemImage: "magnifyingglass.circle.fill",
                value: MainViewTab.searchNotes,
                role: .search
            ) {
                NavigationStack {
                    MarkdownNotesSearchResultsView(
                        editingNote: $editingNote,
                    )
                }
            }
            .customizationID("\(MainViewTab.searchNotes.rawValue)-tabbar")
            .customizationBehavior(.disabled, for: .sidebar)
            .tabPlacement(.pinned)
            TabSection(
                "Search",
                content: {
                    Tab(
                        "Search Notes",
                        systemImage: "magnifyingglass.circle.fill",
                        value: MainViewTab.searchNotes,
                        role: .search
                    ) {
                        NavigationStack {
                            MarkdownNotesSearchResultsView(
                                editingNote: $editingNote,
                            )
                        }
                    }
                    .customizationID(MainViewTab.searchNotes.rawValue)
                    .customizationBehavior(.disabled, for: .tabBar)
                    .defaultVisibility(.hidden, for: .tabBar)
                    Tab(
                        "Search bibliography",
                        systemImage: "magnifyingglass.circle.fill",
                        value: MainViewTab.searchByBib,
                    ) {
                        NavigationStack {
                            BibliographySearchResultsView(
                                editingNote: $editingNote,
                            )
                            .navigationTitle("Bibliography Entries")
                            .navigationDestination(
                                for: BibEntryModel.self,
                                destination: { bibEntry in
                                    ByBibEntrySearchResults(
                                        bibEntryId: bibEntry.id,
                                        editingNote: $editingNote
                                    )
                                }
                            )
                        }
                    }
                    .customizationID(MainViewTab.searchByBib.rawValue)
                    .customizationBehavior(.disabled, for: .tabBar)
                    .defaultVisibility(.hidden, for: .tabBar)
                }
            )
            .customizationID("TabGroup.search")
            .customizationBehavior(.disabled, for: .tabBar)
            .tabPlacement(.sidebarOnly)
            .defaultVisibility(.hidden, for: .tabBar)
            Tab(
                "Settings",
                systemImage: "gearshape.fill",
                value: MainViewTab.settings
            ) {
                SettingsPageView(
                    theme: $theme,
                    editorThemeDark: $editorThemeDark,
                    editorThemeLight: $editorThemeLight,
                    colorSchemeSelection: $colorSchemeSelection
                )
            }
            .tabPlacement(.sidebarOnly)
            .customizationID(MainViewTab.settings.rawValue)
            Tab(
                "Create Note",
                systemImage: "plus",
                value: MainViewTab.createNote
            ) {
                CreateNoteSheetView(editingNote: $editingNote, dismissOnSubmit: false)
            }
            .customizationID(MainViewTab.createNote.rawValue)
            .customizationBehavior(.disabled, for: .tabBar)
            .defaultVisibility(.hidden, for: .tabBar)
        }
        .onChange(
            of: colorScheme,
            {
                handleColorSchemeChange(newScheme: colorScheme)
            }
        )
        .onChange(
            of: webviewFontSize,
            {
                editorContainer.setWebviewFontSize(fontSize: webviewFontSize)
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
                editorContainer.applyWebViewColorScheme(
                    darkMode: colorSchemeSelection == .dark
                )
            }
        )
        .onAppear {
            handleColorSchemeChange(newScheme: colorScheme)
            handleThemeChange(newTheme: theme)
            handleColorSchemeSelectionChange()
        }
        .tabViewCustomization($tabviewCustomization)
        .tabViewStyle(.sidebarAdaptable)
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
        .environment(editingNote)  // TODO: Remove this and all references to it. Cannot use an optional in the environment.
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
        self.editorContainer.setWebviewTheme(theme: newTheme)
        self.themeManager = ThemeManager(
            initialTheme: getTheme(
                themeName: newTheme,
                darkMode: colorScheme == .dark
            )
        )
    }
    func handleColorSchemeChange(newScheme: ColorScheme) {
        editorContainer.emitEditorThemeEvent(
            theme: colorScheme == .dark ? editorThemeDark : editorThemeLight
        )
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
