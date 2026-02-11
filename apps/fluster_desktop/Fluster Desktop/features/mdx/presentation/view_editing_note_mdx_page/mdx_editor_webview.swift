//
//  mdx_editor_webview.swift
//  Fluster
//
//  Created by Andrew on 1/20/26.
//

import FlatBuffers
import FlusterData
import FlusterMdx
import SwiftData
import SwiftUI
import WebKit

struct MdxEditorWebview: View {
  let editingNote: NoteModel
  private var safeNoteBody: String? {
    if editingNote.isDeleted {
      return nil
    } else {
      return editingNote.markdown.body
    }
  }
  @Query(sort: \BibEntryModel.citationKey) private var bibEntries: [BibEntryModel]
  @Binding var webView: WKWebView
  @Environment(\.modelContext) private var modelContext: ModelContext
  @AppStorage(AppStorageKeys.editorKeymap.rawValue) private var editorKeymap: EditorKeymap = .base
  @AppStorage(AppStorageKeys.editorThemeDark.rawValue) private var editorThemeDark:
    CodeSyntaxTheme = .dracula
  @AppStorage(AppStorageKeys.editorThemeLight.rawValue) private var editorThemeLight:
    CodeSyntaxTheme = .materialLight
  var body: some View {
    WebViewContainerView(
      webview: $webView,
      url: Bundle.main.url(
        forResource: "index",
        withExtension: "html",
        subdirectory: "splitview_mdx_editor"
      )!,
      messageHandlerKeys: [
        SplitviewEditorWebviewActions.onTagClick.rawValue,
        SplitviewEditorWebviewActions.onEditorChange.rawValue,
        SplitviewEditorWebviewActions.setWebviewLoaded.rawValue,
        SplitviewEditorWebviewActions.requestSplitviewEditorData.rawValue

      ],
      messageHandler: messageHandler,
      onLoad: onWebviewLoad
    )
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
    .frame(maxWidth: .infinity, maxHeight: .infinity)
  }
  func onWebviewLoad() async {
    Task {
      do {
        try await setEditorThemeDark(editorTheme: editorThemeDark)
        try await setEditorThemeLight(editorTheme: editorThemeLight)
        try await setEditorKeymap(editorKeymap: editorKeymap)
        try await loadNote(note: editingNote)
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
          if let parsedMdx =
            await safeNoteBody?.preParseAsMdxToBytes(noteId: editingNote.id)
          {
            do {
              try await setParsedEditorContent(
                note: editingNote
              )
            } catch {
              print("Error: \(error.localizedDescription)")
            }
            if let parsingResults =
              parsedMdx.toMdxParsingResult()
            {
              editingNote.applyMdxParsingResults(
                results: parsingResults,
                modelContext: modelContext
              )
            }
          }
        }
      default:
        return
    }
  }
  public func handleEditorChange(newValue: String) {
    editingNote.markdown.body = newValue
    editingNote.setLastRead(setModified: true)
    Task {
      do {
        try await setParsedEditorContentString(body: newValue)
      } catch {
        print("Error: \(error.localizedDescription)")
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
    try await webView.evaluateJavaScript(
      """
      window.setCodeSyntaxThemeDark("\(editorTheme.rawValue)")
      """)
  }
  func setEditorThemeLight(editorTheme: CodeSyntaxTheme) async throws {
    try await webView.evaluateJavaScript(
      """
      window.setCodeSyntaxThemeLight("\(editorTheme.rawValue)")
      """)
  }
  func setEditorKeymap(editorKeymap: EditorKeymap) async throws {
    try await webView.evaluateJavaScript(
      """
      window.setEditorKeymap("\(editorKeymap.rawValue)")
      """)
  }
  func setParsedEditorContent(note: NoteModel) async throws {
    if let preParsedBody = note.markdown.preParsedBody {
      try await webView.evaluateJavaScript(
        """
        window.setParsedEditorContentString(\(preParsedBody.toQuotedJavascriptString()))
        """)
    }
  }
  func setParsedEditorContentString(body: String) async throws {
    try await webView.evaluateJavaScript(
      """
      window.setParsedEditorContentString(\(body.toQuotedJavascriptString()))
      """)
  }
  func setEditorContent(note: NoteModel) async throws {
    let body = note.markdown.body.toQuotedJavascriptString()
    try await webView.evaluateJavaScript(
      """
      window.setEditorContent(\(body))
      """)
  }
  func loadNote(note: NoteModel) async throws {
    try await setParsedEditorContent(note: note)
    try await setEditorContent(note: note)
  }
}

#Preview {
  MdxEditorWebview(
    editingNote: NoteModel.fromNoteBody(noteBody: "# My Note"),
    webView: .constant(
      WKWebView(
        frame: .infinite, configuration: getWebViewConfig())
    ))
}
