//
//  dashboard_view.swift
//  Fluster
//
//  Created by Andrew on 10/29/25.
// swiftlint:disable file_length

import FlatBuffers
import FlusterRust
import FlusterSwift
import PencilKit
import SwiftData
import SwiftUI
internal import os

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
  @State private var fullScreenCover: MainFullScreenCover?
  @State private var subjectQuery: String = ""
  @State private var topicQuery: String = ""
  @State private var tagQuery: String = ""
  @AppStorage(AppStorageKeys.hasLaunchedPreviously.rawValue) private
    var hasPreviouslyLaunched: Bool = false
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
  @AppStorage(AppStorageKeys.editorKeymap.rawValue) private var editorKeymap: EditorKeymap = .base
  @AppStorage(AppStorageKeys.tabviewCustomization.rawValue) private
    var tabviewCustomization: TabViewCustomization
  /// editingNoteId is a EditingNoteId.value protocol string. Use that class to parse this result
  /// and retrieve results since this encodes both user defined ids and uuids.
  let defaultNoteId: String = "welcomeToFluster"
  @StateObject private var editorContainer: MdxEditorWebviewContainer =
    MdxEditorWebviewContainer(
      bounce: false,
      scrollEnabled: true
    )
  @State private var themeManager: ThemeManager = ThemeManager(
    initialTheme: getTheme(themeName: getInitialTheme(), darkMode: true)
  )
  /// haveSetNoteDataId is used to set last_read and other data when a user navigates to the note's
  ///  content rather than setting it when the user selects the note.
  @State private var haveSetNoteDataId: String?
  @State private var editingNote: NoteModel?
  @State private var selectedTab: IpadMainViewTab = IpadMainViewTab.notes
  @State private var findInNotePresented: Bool = false

  var body: some View {
    let showFullScreenCover = Binding<Bool>(
      get: { self.fullScreenCover != nil },
      set: { newValue in
        if !newValue {
          self.fullScreenCover = nil
        } else {
          FlusterLogger(.mainApp, .devOnly).log(
            "Attempted to open full screen cover via derived showFullScreenCover.",
            .trace
          )
        }
      }
    )
    TabView(selection: $selectedTab) {
      Tab(
        "Paper",
        systemImage: FlusterIcon.paper.rawValue,
        value: IpadMainViewTab.paper
      ) {
        PaperTabView(
          editingNote: $editingNote,
          selectedTab: $selectedTab
        )
      }
      .customizationID(IpadMainViewTab.paper.rawValue)
      .defaultVisibility(.visible, for: .tabBar)
      Tab(
        "Markdown",
        systemImage: FlusterIcon.markdown.rawValue,
        value: IpadMainViewTab.markdown
      ) {
        MarkdownTabView(
          editingNote: $editingNote,
          editorContainer: editorContainer
        )
      }
      .customizationID(IpadMainViewTab.markdown.rawValue)
      .defaultVisibility(.visible, for: .tabBar)
      Tab(
        "Bibliography",
        systemImage: FlusterIcon.bibliography.rawValue,
        value: IpadMainViewTab.bib
      ) {
        BibliographyTabView(
          editingNote: $editingNote
        )
      }
      .customizationID(IpadMainViewTab.bib.rawValue)
      .defaultVisibility(.visible, for: .tabBar)
      Tab(
        "Details",
        systemImage: FlusterIcon.details.rawValue,
        value: IpadMainViewTab.noteDetail
      ) {
        NoteDetailsTabView(
          editingNote: $editingNote,
          fullScreenCover: $fullScreenCover
        )
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
            systemImage: "book.pages.fill",
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
            "Bookmarks",
            systemImage: FlusterIcon.bookmark.rawValue,
            value: IpadMainViewTab.bookmarks,
          ) {
            NavigationStack {
              BookmarksView(
                editingNote: $editingNote
              )
              .navigationTitle("Bookmarks")
            }
          }
          .customizationID(IpadMainViewTab.bookmarks.rawValue)
          .customizationBehavior(.disabled, for: .tabBar)
          .defaultVisibility(.hidden, for: .tabBar)
          Tab(
            "Bibliography",
            systemImage: FlusterIcon.bibliography.rawValue,
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
            systemImage: FlusterIcon.tag.rawValue,
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
          Tab(
            "Topics",
            systemImage: FlusterIcon.topic.rawValue,
            value: IpadMainViewTab.searchByTopic,
          ) {
            NavigationStack {
              TopicSearchResultListView(
                topicQuery: $topicQuery,
                editingNote: $editingNote
              )
              .navigationTitle("Topics")
            }
          }
          .customizationID(IpadMainViewTab.searchByTopic.rawValue)
          .customizationBehavior(.disabled, for: .tabBar)
          .defaultVisibility(.hidden, for: .tabBar)
          Tab(
            "Subjects",
            systemImage: FlusterIcon.subject.rawValue,
            value: IpadMainViewTab.searchBySubject,
          ) {
            NavigationStack {
              SubjectSearchResultListView(
                subjectQuery: $subjectQuery,
                editingNote: $editingNote
              )
              .navigationTitle("Subjects")
            }
          }
          .customizationID(IpadMainViewTab.searchBySubject.rawValue)
          .customizationBehavior(.disabled, for: .tabBar)
          .defaultVisibility(.hidden, for: .tabBar)
        }
      )
      .customizationID("TabGroup.search")
      .customizationBehavior(.disabled, for: .tabBar)
      .tabPlacement(.sidebarOnly)
      .defaultVisibility(.hidden, for: .tabBar)
      TabSection(
        "Fluster",
        content: {
          Tab(
            "Create Note",
            systemImage: "plus",
            value: IpadMainViewTab.createNote
          ) {
            CreateNoteSheetView(
              editingNote: $editingNote,
              dismissOnSubmit: false,
            )
          }
          .customizationID(IpadMainViewTab.createNote.rawValue)
          .customizationBehavior(.disabled, for: .tabBar)
          .defaultVisibility(.hidden, for: .tabBar)
          Tab(
            "Dictionary",
            systemImage: FlusterIcon.dictionary.rawValue,
            value: IpadMainViewTab.dictionary
          ) {
            DictionaryTab()
          }
          .tabPlacement(.sidebarOnly)
          .customizationID(IpadMainViewTab.dictionary.rawValue)
          .defaultVisibility(.hidden, for: .tabBar)
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
        editorContainer.resetScrollPosition(
          containerId: "mdx-preview",
          scrollStorageKeys: [
            SplitviewEditorWebviewLocalStorageKeys.scrollPositionLandscape.rawValue,
            SplitviewEditorWebviewLocalStorageKeys.scrollPositionPortrait.rawValue
          ])
        if let note = editingNote {
          editorContainer.setInitialContent(note: note)
        }
      }
    )
    .onChange(
      of: editingNote?.markdown.body,
      {
        Task {
          // NOTE: Don't set the last read state with setModified: true here.
          // It's being set in the event to avoid setting it on the initial render.
          if let note = editingNote {
            if let parsedMdx =
              await note.markdown
                .body.preParseAsMdxToBytes(noteId: note.id)
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
                modelContext: modelContext
                )
              }
              Task {
                do {
                  try modelContext.save()
                } catch {
                  print(
                    "Failed to save model context when navigating away from editor view."
                  )
                }
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
      of: selectedTab,
      {
        if let editingNoteBinding = editingNote {
          if haveSetNoteDataId != editingNoteBinding.id {
            if [
              IpadMainViewTab.noteDetail, IpadMainViewTab.markdown,
              IpadMainViewTab.bib, IpadMainViewTab.paper
            ].contains(selectedTab) {
              editingNoteBinding.setLastRead()
              haveSetNoteDataId = editingNoteBinding.id
            }
          }
        }
      }
    )
    .onAppear {
      handleColorSchemeChange(newScheme: colorScheme)
      handleThemeChange(newTheme: theme)
      handleColorSchemeSelectionChange()
      if !hasPreviouslyLaunched {
        Task {
          try? await self.setEditingNoteFromEditingNoteId()
        }
      }
      hasPreviouslyLaunched = true
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
    .fullScreenCover(
      isPresented: showFullScreenCover,
      content: {
        if let fs = fullScreenCover {
          switch fs {
            case .tagSearch(let tag):
              FullScreenSheetDraggableView(
                open: showFullScreenCover,
                content: {
                  NoteSearchResultsByTagView(
                    tag: tag,
                    editingNote: $editingNote
                  )
                }
              )
            case .topicSearch(let topic):
              FullScreenSheetDraggableView(
                open: showFullScreenCover,
                content: {
                  Text("Here")
                  NoteSearchResultsByTopicView(
                    topic: topic,
                    editingNote: $editingNote
                  )
                }
              )
            case .subjectSearch(let subject):
              FullScreenSheetDraggableView(
                open: showFullScreenCover,
                content: {
                  NoteSearchResultsBySubjectView(
                    subject: subject,
                    editingNote: $editingNote
                  )
                }
              )
          }
        } else {
          Text("Something went wrong")
        }
      }
    )
    .environment(\.mainTab, $selectedTab)
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
  func setEditingNoteFromEditingNoteId() async throws {
    let defaultNoteId = self.defaultNoteId
    let fetchDescriptor = FetchDescriptor<NoteModel>(
      predicate: #Predicate<NoteModel> { note in
        note.frontMatter.userDefinedId == defaultNoteId
      }
    )
    let res = try modelContext.fetch(fetchDescriptor)
    if !res.isEmpty && editingNote == nil {
      self.editingNote = res.first
    }
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
