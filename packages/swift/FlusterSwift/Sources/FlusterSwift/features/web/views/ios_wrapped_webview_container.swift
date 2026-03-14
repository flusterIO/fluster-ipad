//
//  File.swift
//  FlusterSwift
//
//  Created by Andrew on 3/12/26.
//

import FlusterData
import SwiftData
import SwiftUI
import WebKit

#if os(iOS)
  public struct IosWebviewContainer: View {
    let implementation: WebviewImplementation
    @Environment(\.modelContext) private var modelContext: ModelContext

    @Query(sort: \BibEntryModel.citationKey) private var bibEntries: [BibEntryModel]
    @AppStorage(AppStorageKeys.editorKeymap.rawValue) private var editorKeymap: CodeEditorKeymap =
      .base
    @AppStorage(AppStorageKeys.editorThemeDark.rawValue) private var editorThemeDark:
      CodeEditorTheme = .dracula
    @AppStorage(AppStorageKeys.editorThemeLight.rawValue) private var editorThemeLight:
      CodeEditorTheme = .materialLight
    @AppStorage(AppStorageKeys.lockEditorScrollToPreview.rawValue) private
      var lockEditorScrollToPreview: Bool = false
    @AppStorage(AppStorageKeys.editorSaveMethod.rawValue) private var editorSaveMethod:
      EditorSaveMethod = .onChange
    @AppStorage(AppStorageKeys.embeddedCslFile.rawValue) private var cslFile: EmbeddedCslFileSwift =
      .apa
    @AppStorage(AppStorageKeys.theme.rawValue) private var flusterTheme: FlusterTheme = .fluster
    @AppStorage(AppStorageKeys.includeEmojiSnippets.rawValue) private var includeEmojiSnippets:
      Bool =
        true
    @Binding public var editingNote: NoteModel?
    @Binding var webView: WKWebView
    public let url: URL
    public let messageHandlerKeys: [String]
    public let messageHandler: ((String, Any) -> Void)?
    public let onLoad: (@Sendable () async -> Void)?
    @AppStorage(AppStorageKeys.theme.rawValue) private var theme: FlusterTheme = .fluster
    @Environment(\.colorScheme) private var colorScheme: ColorScheme
    @Binding public var show: Bool

    public init(
      implementation: WebviewImplementation,
      editingNote: Binding<NoteModel?>,
      webView: Binding<WKWebView>,
      show: Binding<Bool>,
      url: URL,
      messageHandlerKeys: [String],
      messageHandler: ((String, Any) -> Void)?,
      onLoad: (@Sendable () async -> Void)?
    ) {
      self.implementation = implementation
      self._editingNote = editingNote
      self._webView = webView
      self._show = show
      self.url = url
      self.messageHandlerKeys = messageHandlerKeys
      self.messageHandler = messageHandler
      self.onLoad = onLoad
    }

    public var body: some View {
      WebviewContainer(
        parent: self,
        webView: $webView,
        show: $show,
        url: url,
        messageHandlerKeys: messageHandlerKeys,
        messageHandler: messageHandler,
        onLoad: onLoadWrapper
      )
      .onChange(
        of: editingNote?.id,
        {
          Task(priority: .high) {
            await handleInitialState()
          }
        }
      )
      .onChange(
        of: editorKeymap,
        {
          Task(priority: .high) {
            try? await EditorState.setEditorKeymap(
              keymap: editorKeymap, eval: webView.evaluateJavaScript)
          }
        }
      )
      .onChange(
        of: editingNote?.markdown.preParsedBody,
        {
          if editorSaveMethod == .onChange {
            updateParsedEditorValue()
          }
        }
      )
      .onChange(
        of: editorThemeDark,
        {
          do {
            Task(
              priority: .high,
              operation: {
                try await self.setEditorThemeDark(editorTheme: editorThemeDark)
              })
          }
        }
      )
      .onChange(
        of: editorThemeLight,
        {
          do {
            Task(
              priority: .high,
              operation: {
                try await self.setEditorThemeLight(editorTheme: editorThemeLight)
              })
          }
        }
      )
      .onChange(
        of: colorScheme,
        {
          Task {
            await setColorScheme(colorScheme: colorScheme)
          }
        }
      )
    }
    func setColorScheme(colorScheme: ColorScheme) async {
      try? await EditorState.setDarkMode(colorScheme: colorScheme, eval: webView.evaluateJavaScript)
    }
    func updateParsedEditorValue() {
      if let en = editingNote {
        Task(priority: .high) {
          try? await en.preParseIfEdited(modelContext: modelContext)
          let citations: [EditorCitation] = en.citations.compactMap { cit in
            cit.toEditorCitation(activeCslFile: cslFile)
          }
          try? await EditorState.setParsedMdxContent(
            parsedMdxContent: en.markdown.preParsedBody ?? "", citations: citations,
            eval: webView.evaluateJavaScript)
        }
      }
    }

    func setEditorThemeDark(editorTheme: CodeEditorTheme) async throws {
      try await EditorState.setEditorThemeDark(
        payload: SetEditorThemeDarkPayload(theme_dark: editorTheme),
        eval: self.webView.evaluateJavaScript
      )
    }
    func setEditorThemeLight(editorTheme: CodeEditorTheme) async throws {
      try await EditorState.setEditorThemeLight(
        payload: SetEditorThemeLightPayload(theme_light: editorTheme),
        eval: self.webView.evaluateJavaScript
      )
    }
    public func handleInitialState() async {
      if let en = editingNote {
        Task(priority: .high) {
          do {
            try await en.preParse(modelContext: modelContext)
            try await EditorState.setInitialEditorState(
              editorPayload: EditorInitialStatePayload(
                note_id: en.id,
                keymap: editorKeymap,
                theme_light: editorThemeLight,
                theme_dark: editorThemeDark,
                allCitationIds: bibEntries.compactMap(\.citationKey),
                value: en.markdown.body,
                parsedValue: en.markdown.preParsedBody ?? "",
                haveSetInitialValue: true,
                snippetProps: SnippetState(
                  includeEmojiSnippets: includeEmojiSnippets
                ),
                lockEditorScrollToPreview: lockEditorScrollToPreview,
                saveMethod: editorSaveMethod
              ),
              containerPayload: WebviewContainerSharedInitialState(
                environment: WebviewEnvironment.iPad,
                dark_mode: colorScheme == .dark,
                implementation: self.implementation,
                fluster_theme: flusterTheme
              ),
              eval: self.webView.evaluateJavaScript
            )
          } catch {
            print("Error initializing Mdx Editor Webview: \(error.localizedDescription)")
          }
        }
      }
    }
    func onLoadWrapper() {
      Task(priority: .high) {
        await handleInitialState()
      }
      if let onLoad = self.onLoad {
        Task(priority: .userInitiated) {
          await onLoad()
        }
      }
    }
  }
#endif
