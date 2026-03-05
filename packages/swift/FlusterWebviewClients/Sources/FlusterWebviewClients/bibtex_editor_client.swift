import Foundation
import FlusterData


public class BibtexEditorClient {
    public static func emitEditorThemeEvent(theme: CodeEditorTheme, evalutateJavaScript: EvalJavascriptFunc) async throws {
    try await evalutateJavaScript(
      """
      window.localStorage.setItem("\(BibtexEditorWebviewEvents.setCodeTheme.rawValue)", "\(theme.rawValue)")
      window.setCodeSyntaxTheme("\(theme.rawValue)")
      """
    )
  }
  public static func setEditorKeymap(editorKeymap: CodeEditorKeymap, evalutateJavaScript: EvalJavascriptFunc ) async throws {
    try await evalutateJavaScript(
      """
      window.localStorage.setItem("\(BibtexEditorWebviewEvents.setEditorKeymap.rawValue)", "\(editorKeymap.rawValue)")
      window.setEditorKeymap("\(editorKeymap.rawValue)")
      """
    )
  }
  public static func clearEditorData(evalutateJavaScript: EvalJavascriptFunc ) async throws {
    try await evalutateJavaScript(
      """
      window.clearBibtexEditorData()
      """)
  }
  public static func setBibEntryContent(entryBody: String, evalutateJavaScript: EvalJavascriptFunc ) async throws {
    let body = entryBody.toFlatBufferSerializedString()
    try await evalutateJavaScript(
      """
      window.setBibtexEditorContent(\(body))
      """
    )
  }
}
