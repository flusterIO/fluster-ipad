//
//  mdx_webview_editor_extension.swift
//  Fluster
//
//  Created by Andrew on 3/5/26.
//

import FlusterData
import FlusterWebviewClients
import SwiftUI
import WebKit

extension MdxEditorWebview {
  func setEditorThemeDark(editorTheme: CodeEditorTheme) async throws {
    try await MdxEditorClient.setEditorThemeDark(
      editorTheme: editorTheme, evaluateJavaScript: webView.evaluateJavaScript)
    if colorScheme == .dark {
      try await setEditorSelectedTheme(editorTheme: editorTheme)
    }
  }
  func setEditorThemeLight(editorTheme: CodeEditorTheme) async throws {
    try await MdxEditorClient.setEditorThemeLight(
      editorTheme: editorTheme, evaluateJavaScript: webView.evaluateJavaScript)
    if colorScheme == .light {
      try await setEditorSelectedTheme(editorTheme: editorTheme)
    }
  }
  func setEditorSelectedTheme(editorTheme: CodeEditorTheme) async throws {
    try await EditorState.setEditorTheme(
      payload: SetEditorThemePayload(theme: editorTheme), eval: webView.evaluateJavaScript)
  }
  func setCodeEditorKeymap(editorKeymap: CodeEditorKeymap) async throws {
    try await EditorState.setEditorKeymap(keymap: editorKeymap, eval: webView.evaluateJavaScript)
  }
  func setEditorContent(note: NoteModel) async throws {
    try await EditorState.setEditorContent(
      content: note.markdown.body,
      eval: webView.evaluateJavaScript
    )
  }
  func setLockEditorScrollToPreview(lock: Bool) async throws {
    try await EditorState.setLockEditorScrollToPreview(
      payload: SetLockEditorScrollToPreviewPayload(lock_editor_scroll_to_preview: lock),
      eval: webView.evaluateJavaScript)
  }
  /// deprecated: Moving to new Redux setup.
  func loadNote(note: NoteModel) async throws {
    try await setEditorContent(note: note)
  }
}
