//
//  bibtex_editor_webview_container.swift
//  Fluster
//
//  Created by Andrew on 11/22/25.
//

import Foundation
import FlusterData

#if os(iOS)
public final class BibtexEditorWebviewContainer: WebviewContainer<BibtexEditorWebviewEvents> {
  //    public override init() {}
  public func emitEditorThemeEvent(theme: CodeSyntaxTheme) {
    print("Changing editor theme event")
    self.runJavascript(
      """
      window.localStorage.setItem("\(BibtexEditorWebviewEvents.setCodeTheme.rawValue)", "\(theme.rawValue)")
      window.setCodeSyntaxTheme("\(theme.rawValue)")
      """
    )
  }
  public func setEditorKeymap(editorKeymap: EditorKeymap) {
    print("Applying editor keymap")
    self.runJavascript(
      """
      window.localStorage.setItem("\(BibtexEditorWebviewEvents.setEditorKeymap.rawValue)", "\(editorKeymap.rawValue)")
      window.setEditorKeymap("\(editorKeymap.rawValue)")
      """
    )
  }
  public func clearEditorData() {
    self.runJavascript(
      """
      window.clearBibtexEditorData()
      """)
  }
  public func setInitialContent(entryBody: String) {
    let body = entryBody.toFlatBufferSerializedString()
    self.runJavascript(
      """
      window.setBibtexEditorContent(\(body))
      """
    )
  }
  public func setInitialProperties(
    initialValue: String?,
    codeEditorTheme: CodeSyntaxTheme,
    editorKeymap: EditorKeymap,
    theme: WebViewTheme,
    fontSize: WebviewFontSize,
    darkMode: Bool
  ) {
    self.applyWebViewColorScheme(darkMode: darkMode)
    self.emitEditorThemeEvent(theme: codeEditorTheme)
    self.setEditorKeymap(editorKeymap: editorKeymap)
    self.setWebviewTheme(theme: theme)
    self.setWebviewFontSize(fontSize: fontSize)
    self.setInitialContent(entryBody: initialValue ?? "")
  }
}
#endif
