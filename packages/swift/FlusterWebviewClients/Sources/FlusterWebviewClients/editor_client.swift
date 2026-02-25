//
//  File.swift
//  FlusterMdx
//
//  Created by Andrew on 2/12/26.
//

import FlusterData
import Foundation
import SwiftData

public typealias EvalJavascriptFunc = @Sendable (String) async throws -> Sendable?

@MainActor
public struct MdxEditorClient {
  public static func setEditorKeymap(keymap: EditorKeymap, evaluateJavaScript: EvalJavascriptFunc)
    async throws
  {
    try await evaluateJavaScript(
      """
      window.setEditorKeymap("\(keymap.rawValue)")
      """)
  }
  public static func setEditorThemeLight(
    editorTheme: CodeSyntaxTheme, evaluateJavaScript: EvalJavascriptFunc
  ) async throws {
    try await evaluateJavaScript(
      """
      window.setCodeSyntaxThemeLight("\(editorTheme.rawValue)")
      """)
  }
  public static func setEditorThemeDark(
    editorTheme: CodeSyntaxTheme, evaluateJavaScript: EvalJavascriptFunc
  ) async throws {
    try await evaluateJavaScript(
      """
      window.setCodeSyntaxThemeDark("\(editorTheme.rawValue)")
      """)
  }
  public static func setEditorTheme(
    editorTheme: CodeSyntaxTheme, evaluateJavaScript: @escaping EvalJavascriptFunc
  ) async throws {
    try await evaluateJavaScript(
      """
      window.localStorage.setItem("\(SplitviewEditorWebviewLocalStorageKeys.codeTheme.rawValue)", "\(editorTheme.rawValue)")
      window.setCodeSyntaxTheme("\(editorTheme.rawValue)")
      """)
  }
  public static func setParsedEditorContent(
    note: NoteModel, evaluateJavaScript: @escaping EvalJavascriptFunc
  ) async throws {
    if let preParsedBody = note.markdown.preParsedBody {
      try await evaluateJavaScript(
        """
        window.setParsedEditorContentString(\(preParsedBody.toFlatBufferSerializedString()))
        """)
    }
  }

  public static func setEditorContent(
    note: NoteModel,
    evaluateJavaScript: @escaping EvalJavascriptFunc
  ) async throws {
    let body = note.markdown.body.toFlatBufferSerializedString()
    try await evaluateJavaScript(
      """
      window.setEditorContent(\(body))
      """)
  }

  public static func setLockEditorScrollToPreview(
    _ lock: Bool, evaluateJavaScript: @escaping EvalJavascriptFunc
  ) async throws {
    try await evaluateJavaScript(
      """
      window.setLockEditorScrollToPreview(\(lock ? "true" : "false"))
      """)
  }
}
