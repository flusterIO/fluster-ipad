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
  /// Pass in an un-parsed string for the data. It will be converted to a javascript compliant string internally.
  public static func saveToLocalStorage(
    storageKey: String, data: String, evaluateJavaScript: EvalJavascriptFunc
  ) async throws {
    try await evaluateJavaScript(
      """
      window.localStorage.setItem("\(storageKey)", \(data.toQuotedJavascriptString()))
      window.dispatchEvent(
          new CustomEvent("\(SharedWebviewEvents.localStorageWrite.rawValue)", {
              detail: null
          })
      )
      """)
  }
  public static func setEditorThemeLight(
    editorTheme: CodeEditorTheme, evaluateJavaScript: EvalJavascriptFunc
  ) async throws {
    try await evaluateJavaScript(
      """
      window.setCodeSyntaxThemeLight("\(editorTheme.rawValue)")
      """)
  }
  public static func setEditorThemeDark(
    editorTheme: CodeEditorTheme, evaluateJavaScript: EvalJavascriptFunc
  ) async throws {
    try await evaluateJavaScript(
      """
      window.setCodeSyntaxThemeDark("\(editorTheme.rawValue)")
      """)
  }
  public static func setEditorTheme(
    editorTheme: CodeEditorTheme, evaluateJavaScript: @escaping EvalJavascriptFunc
  ) async throws {
    try await evaluateJavaScript(
      """
      window.localStorage.setItem("\(SplitviewEditorWebviewLocalStorageKeys.codeTheme.rawValue)", "\(editorTheme.rawValue)")
      window.setCodeSyntaxTheme("\(editorTheme.rawValue)")
      """)
  }

  /// Sets the actual editor's content, not the pre-parsed preview content.
  public static func setEditorContent(
    note: NoteModel,
    evaluateJavaScript: @escaping EvalJavascriptFunc
  ) async throws {
    try await self.saveToLocalStorage(
      storageKey: SplitviewEditorWebviewLocalStorageKeys.initialValue.rawValue,
      data: note.markdown.body,
      evaluateJavaScript: evaluateJavaScript)
  }

  public static func setLockEditorScrollToPreview(
    _ lock: Bool, evaluateJavaScript: @escaping EvalJavascriptFunc
  ) async throws {
    try await evaluateJavaScript(
      """
      window.setLockEditorScrollToPreview(\(lock ? "true" : "false"))
      """)
  }
  // -- New Editor State Model --
  /// Takes a stringified `AnyCrossLanguageEditorAction` to be sent to the cross-language state update handler.
  public static func sendEditorStateUpdate(
    data: String, evalulateJavaScript: @escaping EvalJavascriptFunc
  ) async throws {
    let res = try await evalulateJavaScript(
      """
      try {
      window.handleSwiftAction('\(data)')
      } catch (err) {
      console.error("Swift Action Error: ", err)
      }
      """)
    print("Res: \(res)")
  }
}
