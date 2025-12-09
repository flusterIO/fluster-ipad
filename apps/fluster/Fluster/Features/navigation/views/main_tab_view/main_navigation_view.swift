//
//  dashboard_view.swift
//  Fluster
//
//  Created by Andrew on 10/29/25.
//

import FlatBuffers
import FlusterRust
import FlusterSwift
import PencilKit
import SwiftData
import SwiftUI

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

@MainActor
struct MainView: View {
    @State private var toolbar = PKToolPicker()
    @State private var tagQuery: String = ""
    @AppStorage(AppStorageKeys.theme.rawValue) private var theme: WebViewTheme =
        .fluster
    @AppStorage(AppStorageKeys.webviewFontSize.rawValue) private
        var webviewFontSize: WebviewFontSize = .base
    @AppStorage(AppStorageKeys.silenceParsingErrors.rawValue) private
        var silenceParsingErrors: Bool = false
    @Environment(\.colorScheme) var colorScheme: ColorScheme
    @Environment(\.modelContext) var modelContext: ModelContext
    @AppStorage(AppStorageKeys.colorScheme.rawValue) private
        var colorSchemeSelection: ColorSchemeSelection = .dark
    @AppStorage(AppStorageKeys.editorThemeDark.rawValue) private
        var editorThemeDark: CodeSyntaxTheme = .dracula
    @AppStorage(AppStorageKeys.editorThemeLight.rawValue) private
        var editorThemeLight: CodeSyntaxTheme = .githubLight
    @AppStorage(AppStorageKeys.editorKeymap.rawValue) private var editorKeymap:
        EditorKeymap = .base
    @AppStorage(AppStorageKeys.tabviewCustomization.rawValue) private
        var tabviewCustomization: TabViewCustomization
    @StateObject private var editorContainer = MdxEditorWebviewContainer(
        bounce: false,
        scrollEnabled: true
    )
    @State private var themeManager = ThemeManager(
        initialTheme: getTheme(themeName: getInitialTheme(), darkMode: true)
    )
    @State private var selectedTab = IpadMainViewTab.paper
    @State private var editingNoteId: String?
    @State private var editingNote: NoteModel?
    @State private var findInNotePresented: Bool = false

    var body: some View {
        TabView(selection: $selectedTab) {
            Tab(
                "Paper",
                systemImage: "pencil.circle.fill",
                value: IpadMainViewTab.paper
            ) {
                if let unwrappedEditingNote = Binding($editingNote) {
                    PaperView(
                        toolbar: $toolbar,
                        drawingData: unwrappedEditingNote.drawing,
                        activeTab: $selectedTab
                    )
                    .backgroundExtensionEffect()
                    .ignoresSafeArea()
                } else {
                    SelectNoteToContinueView()
                }
            }
            .customizationID(IpadMainViewTab.paper.rawValue)
            .defaultVisibility(.visible, for: .tabBar)
            Tab(
                "Markdown",
                systemImage: "book.closed.circle.fill",
                value: IpadMainViewTab.markdown
            ) {
                if editingNote != nil {
                    MdxEditorWebview(
                        url:
                            Bundle.main.url(
                                forResource: "index",
                                withExtension: "html",
                                subdirectory: "splitview_mdx_editor"
                            )!,
                        theme: $theme,
                        editorThemeDark: $editorThemeDark,
                        editorThemeLight: $editorThemeLight,
                        editingNote: $editingNote,
                        editorKeymap: $editorKeymap,
                        container: editorContainer,
                    )
                    //                    .moveDisabled(true)
                    .disableAnimations()
                    //                    .scrollDisabled(true)
                    .frame(
                        alignment: .bottom
                    )
                    // TODO: Remove this. This is just for easy development.
                    .scrollDisabled(true)
                    .onAppear {
                        if let parsedMdx = editingNote?.markdown.preParsedBody {
                            editorContainer.setParsedEditorContentString(
                                content: parsedMdx
                            )
                        }
                        UIScrollView.appearance().bounces = false
                        UIScrollView.appearance().isScrollEnabled = false
                    }
                    .onDisappear {
                        UIScrollView.appearance().bounces = true
                        UIScrollView.appearance().isScrollEnabled = true
                    }
                } else {
                    SelectNoteToContinueView()
                }
            }
            .customizationID(IpadMainViewTab.markdown.rawValue)
            .defaultVisibility(.visible, for: .tabBar)
            Tab(
                "Bibliography",
                systemImage: "books.vertical.circle.fill",
                value: IpadMainViewTab.bib
            ) {
                NavigationStack {
                    BibliographyPageView(
                        editingNote: $editingNote
                    )
                }
            }
            .customizationID(IpadMainViewTab.bib.rawValue)
            .defaultVisibility(.visible, for: .tabBar)
            Tab(
                "Details",
                systemImage: "receipt.fill",
                value: IpadMainViewTab.noteDetail
            ) {
                if let noteBinding = Binding($editingNote) {
                    NavigationStack {
                        NoteDetailWebview(
                            note: noteBinding,
                        )
                    }
                } else {
                    SelectNoteToContinueView()
                }
            }
            .customizationID(IpadMainViewTab.noteDetail.rawValue)
            .defaultVisibility(.visible, for: .tabBar)
            Tab(
                "Search",
                systemImage: "magnifyingglass.circle.fill",
                value: IpadMainViewTab.searchNotes,
                role: .search
            ) {
                NavigationStack {
                    MarkdownNotesSearchResultsView(
                        editingNote: $editingNote,
                    )
                }
            }
            .customizationID("\(IpadMainViewTab.searchNotes.rawValue)-tabbar")
            .customizationBehavior(.disabled, for: .sidebar)
            .tabPlacement(.pinned)
            TabSection(
                "Search",
                content: {
                    Tab(
                        "Notes",
                        systemImage: "magnifyingglass.circle.fill",
                        value: IpadMainViewTab.searchNotes,
                        role: .search
                    ) {
                        NavigationStack {
                            MarkdownNotesSearchResultsView(
                                editingNote: $editingNote,
                            )
                        }
                    }
                    .customizationID(IpadMainViewTab.searchNotes.rawValue)
                    .customizationBehavior(.disabled, for: .tabBar)
                    .defaultVisibility(.hidden, for: .tabBar)
                    Tab(
                        "Bibliography",
                        systemImage: "magnifyingglass.circle.fill",
                        value: IpadMainViewTab.searchByBib,
                    ) {
                        NavigationStack {
                            BibliographySearchResultsView(
                                editingNote: $editingNote
                            )
                            .navigationTitle("Bibliography Entries")
                        }
                    }
                    .customizationID(IpadMainViewTab.searchByBib.rawValue)
                    .customizationBehavior(.disabled, for: .tabBar)
                    .defaultVisibility(.hidden, for: .tabBar)
                    Tab(
                        "Tags",
                        systemImage: "magnifyingglass.circle.fill",
                        value: IpadMainViewTab.searchByTag,
                    ) {
                        NavigationStack {
                            TagSearchResultList(
                                tagQuery: $tagQuery,
                                editingNote: $editingNote
                            )
                            .navigationTitle("Tags")
                        }
                    }
                    .customizationID(IpadMainViewTab.searchByTag.rawValue)
                    .customizationBehavior(.disabled, for: .tabBar)
                    .defaultVisibility(.hidden, for: .tabBar)
                }
            )
            .customizationID("TabGroup.search")
            .customizationBehavior(.disabled, for: .tabBar)
            .tabPlacement(.sidebarOnly)
            .defaultVisibility(.hidden, for: .tabBar)
            Tab(
                "Create Note",
                systemImage: "plus",
                value: IpadMainViewTab.createNote
            ) {
                NavigationStack {
                    CreateNoteSheetView(
                        editingNote: $editingNote,
                        dismissOnSubmit: false
                    )
                    .navigationTitle("Create note")
                }
            }
            .customizationID(IpadMainViewTab.createNote.rawValue)
            .customizationBehavior(.disabled, for: .tabBar)
            .defaultVisibility(.hidden, for: .tabBar)
            TabSection(
                "Fluster",
                content: {
                    Tab(
                        "Settings",
                        systemImage: "gearshape.fill",
                        value: IpadMainViewTab.settings
                    ) {
                        SettingsPageView()
                    }
                    .tabPlacement(.sidebarOnly)
                    .customizationID(IpadMainViewTab.settings.rawValue)
                }
            )
        }
        .onChange(
            of: editingNote,
            {
                if let note = editingNote {
                    note.setLastRead(setModified: false)
                    editorContainer.resetScrollPosition()
                    editorContainer.setInitialContent(note: note)
                }
            }
        )
        .onChange(
            of: editingNote?.markdown.body,
            {
                Task {
                    // NOTE: Don't set the last read state with setModified: true here. It's being set in the event to avoid setting it on the initial render.
                    if let note = editingNote {
                        if let parsedMdx =
                            await note.markdown
                            .body.preParseAsMdxToBytes()
                        {
                            editorContainer.emitMdxParsingSuccess()
                            editorContainer.setParsedEditorContent(
                                content: parsedMdx
                            )
                            if let parsingResults =
                                parsedMdx.toMdxParsingResult()
                            {
                                note.applyMdxParsingResults(
                                    results: parsingResults,
                                )
                            }
                        } else {
                            if !silenceParsingErrors {
                                editorContainer.emitMdxParsingError()
                            }
                            print("Could not parse mdx.")
                        }
                    }
                }
            }
        )
        //        .onChange( TODO: this will run every time the note changes and always update it. This will need to be tapped in to some sort of event listener.
        //            of: editingNote?.drawing,
        //            {
        //                if let note = editingNote {
        //                    note.setLastRead(setModified: true)
        //                }
        //            }
        //        )
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
        .onChange(
            of: editingNote,
            {
                if let _editingNote = editingNote {
                    _editingNote.last_read = .now
                }
            }
        )
        .onAppear {
            handleColorSchemeChange(newScheme: colorScheme)
            handleThemeChange(newTheme: theme)
            handleColorSchemeSelectionChange()
            print("View Database: ", modelContext.sqliteCommand)
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
        .environment(editingNote)
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
    func getParseMdxTaskId() -> String {
        if let editingNoteExists = editingNote {
            return "\(editingNoteExists.id)-\(editingNoteExists.markdown.body)"
        }
        return "parse-mdx"
    }
}

#Preview {
    MainView()
        .environment(ThemeManager(initialTheme: FlusterDark()))
}
