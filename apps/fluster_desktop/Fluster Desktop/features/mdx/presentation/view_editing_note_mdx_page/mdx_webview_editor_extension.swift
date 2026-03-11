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
