//
//  mdx_editor_webview.swift
//  Fluster
//
//  Created by Andrew on 1/20/26.
//

import FlatBuffers
import FlusterData
import FlusterMdx
import FlusterWebviewClients
import SwiftData
import SwiftUI
import WebKit

struct MdxEditorWebview: View {
  var editingNoteId: String?
  @Environment(\.modelContext) private var modelContext
  @Environment(\.colorScheme) private var colorScheme
  @EnvironmentObject private var appState: AppState
  @Query private var notes: [NoteModel]

  var editingNote: NoteModel? {
    notes.isEmpty ? nil : notes.first!
  }
  @Query(sort: \BibEntryModel.citationKey) private var bibEntries: [BibEntryModel]
  @Binding var webView: WKWebView
  @AppStorage(AppStorageKeys.editorKeymap.rawValue) private var editorKeymap: EditorKeymap = .base
  @AppStorage(AppStorageKeys.editorThemeDark.rawValue) private var editorThemeDark:
    CodeSyntaxTheme = .dracula
  @AppStorage(AppStorageKeys.editorThemeLight.rawValue) private var editorThemeLight:
    CodeSyntaxTheme = .materialLight
  @AppStorage(AppStorageKeys.lockEditorScrollToPreview.rawValue) private
    var lockEditorScrollToPreview: Bool = false

  init(editingNoteId: String?, webView: Binding<WKWebView>) {
    self.editingNoteId = editingNoteId
    self._webView = webView
    if let _id = editingNoteId {
      let predicate = #Predicate<NoteModel> { $0.id == _id }
      _notes = Query(filter: predicate)
    } else {
      _notes = Query(
        filter: #Predicate<NoteModel> { note in
          false
        })
    }
  }

  var body: some View {
    if editingNoteId == nil {
      NoNoteSelectedView()
    } else {
      WebViewContainerView(
        webview: $webView,
        url: Bundle.main.url(
          forResource: "index",
          withExtension: "html",
          subdirectory: "splitview_mdx_editor"
        )!,
        messageHandlerKeys: [
          MdxPreviewWebviewActions.onTagClick.rawValue,
          SplitviewEditorWebviewActions.onEditorChange.rawValue,
          SplitviewEditorWebviewActions.setWebviewLoaded.rawValue,
          SplitviewEditorWebviewActions.requestSplitviewEditorData.rawValue
        ],
        messageHandler: messageHandler,
        onLoad: onWebviewLoad
      )
      .onAppear {
        if let en = editingNote {
          en.setLastRead()
        }
      }
      .onChange(
        of: editorKeymap,
        {
          Task {
            try? await setEditorKeymap(editorKeymap: editorKeymap)
          }
        }
      )
      .onChange(
        of: editorThemeDark,
        {
          Task {
            try? await setEditorThemeDark(editorTheme: editorThemeDark)
          }
        }
      )
      .onChange(
        of: editorThemeLight,
        {
          Task {
            try? await setEditorThemeLight(editorTheme: editorThemeLight)
          }
        }
      )
      .onChange(
        of: colorScheme,
        {
          Task {
            try? await setEditorSelectedTheme(
              editorTheme: colorScheme == .dark ? editorThemeDark : editorThemeLight)
          }
        }
      )
      .onChange(
        of: lockEditorScrollToPreview,
        {
          Task {
            try? await setLockEditorScrollToPreview(lock: lockEditorScrollToPreview)
          }
        }
      )
      .onChange(
        of: editingNote?.id,
        {
          if let en = editingNote {
            en.setLastRead()
          }
        }
      )
      .frame(maxWidth: .infinity, maxHeight: .infinity)
    }
  }
  func onWebviewLoad() async {
    Task {
      do {
        try await setEditorThemeDark(editorTheme: editorThemeDark)
        try await setEditorThemeLight(editorTheme: editorThemeLight)
        try await setEditorKeymap(editorKeymap: editorKeymap)
        try await setLockEditorScrollToPreview(lock: lockEditorScrollToPreview)
        if let en = editingNote {
          try await loadNote(note: en)
        }
        try await setSnippetProps()
        print("Loaded initial editor data")
      } catch {
        print("Error initalizing Mdx Editor Webview: \(error.localizedDescription)")
      }
    }
  }
  public func messageHandler(_ handlerKey: String, _ messageBody: Any) {
    switch handlerKey {
      case SplitviewEditorWebviewActions.onEditorChange.rawValue:
        handleEditorChange(newValue: messageBody as! String)
      case SplitviewEditorWebviewActions.requestSplitviewEditorData.rawValue:
        Task(priority: .high) {
          await onWebviewLoad()
        }
      case SplitviewEditorWebviewActions.requestParsedMdxContent.rawValue:
        Task(priority: .high) {
          if let en = editingNote {
            do {
              try await setParsedEditorContentString(
                note: en
              )
            } catch {
              print("Error: \(error.localizedDescription)")
            }
          }
        }
      default:
        return
    }
  }
  public func handleEditorChange(newValue: String) {
    if let en = editingNote {
      en.markdown.body = newValue
      en.setLastRead(setModified: true)
      Task {
        try await en.preParse(modelContext: modelContext)
        do {
          try await setParsedEditorContentString(note: en)
        } catch {
          print("Error: \(error.localizedDescription)")
        }
      }
    }
  }

  func setSnippetProps() async throws {
    var builder = FlatBufferBuilder(initialSize: 1024)
    let ctiationIdsVectorOffset = builder.createVector(
      ofStrings: bibEntries.compactMap(\.citationKey))
    let data = Snippets_GetSnippetPropsBuffer.createGetSnippetPropsBuffer(
      &builder, citationIdsVectorOffset: ctiationIdsVectorOffset)
    builder.finish(offset: data)
    let bytes: [UInt8] = Array(builder.data)
    try await webView.evaluateJavaScript(
      """
      window.setSnippetProps(\(bytes))
      """)
  }
  func setEditorThemeDark(editorTheme: CodeSyntaxTheme) async throws {
    try await MdxEditorClient.setEditorThemeDark(
      editorTheme: editorTheme, evaluateJavaScript: webView.evaluateJavaScript)
    if colorScheme == .dark {
      try await setEditorSelectedTheme(editorTheme: editorTheme)
    }
  }
  func setEditorThemeLight(editorTheme: CodeSyntaxTheme) async throws {
    try await MdxEditorClient.setEditorThemeLight(
      editorTheme: editorTheme, evaluateJavaScript: webView.evaluateJavaScript)
    if colorScheme == .light {
      try await setEditorSelectedTheme(editorTheme: editorTheme)
    }
  }
  func setEditorSelectedTheme(editorTheme: CodeSyntaxTheme) async throws {
    try await MdxEditorClient.setEditorTheme(
      editorTheme: editorTheme, evaluateJavaScript: webView.evaluateJavaScript)
  }
  func setEditorKeymap(editorKeymap: EditorKeymap) async throws {
    try await MdxEditorClient.setEditorKeymap(
      keymap: editorKeymap, evaluateJavaScript: webView.evaluateJavaScript)
  }
  func setParsedEditorContentString(note: NoteModel) async throws {
    try await MdxEditorClient.setParsedEditorContentString(
      note: note, evaluateJavaScript: webView.evaluateJavaScript)
  }
  func setEditorContent(note: NoteModel) async throws {
    try await MdxEditorClient.setEditorContent(
      note: note, evaluateJavaScript: webView.evaluateJavaScript)
  }
  func setLockEditorScrollToPreview(lock: Bool) async throws {
    try await MdxEditorClient.setLockEditorScrollToPreview(
      lock, evaluateJavaScript: webView.evaluateJavaScript)
  }
  func loadNote(note: NoteModel) async throws {
    try await setParsedEditorContentString(note: note)
    try await setEditorContent(note: note)
  }
}

#Preview {
  MdxEditorWebview(
    editingNoteId: nil,
    webView: .constant(
      WKWebView(
        frame: .infinite, configuration: getWebViewConfig())
    ))
}
