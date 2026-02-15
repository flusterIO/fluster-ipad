import Combine
import FlatBuffers
import FlusterData
import SwiftData
import SwiftUI
import WebKit
import FlusterWebviewClients

#if os(iOS)
  @MainActor
  public final class MdxEditorWebviewContainer: WebviewContainer<SplitviewEditorWebviewEvents> {
    public func emitEditorThemeEvent(theme: CodeSyntaxTheme) {
      self.runJavascript(
        """
        window.localStorage.setItem("\(SplitviewEditorWebviewLocalStorageKeys.codeTheme.rawValue)", "\(theme.rawValue)")
        window.setCodeSyntaxTheme("\(theme.rawValue)")
        """
      )
    }
    public func setEditorLightTheme(theme: CodeSyntaxTheme) {
      self.runJavascript(
        """
        window.localStorage.setItem("\(SplitviewEditorWebviewLocalStorageKeys.codeThemeLight.rawValue)", "\(theme.rawValue)")
        window.setCodeSyntaxThemeLight("\(theme.rawValue)")
        """
      )
    }
    public func setEditorDarkTheme(theme: CodeSyntaxTheme) {
        
      self.runJavascript(
        """
        window.localStorage.setItem("\(SplitviewEditorWebviewLocalStorageKeys.codeThemeDark.rawValue)", "\(theme.rawValue)")
        window.setCodeSyntaxThemeDark("\(theme.rawValue)")
        """
      )
    }
    public func setEditorKeymap(editorKeymap: EditorKeymap) {
      self.runJavascript(
        """
        window.localStorage.setItem("\(SplitviewEditorWebviewLocalStorageKeys.editorKeymap.rawValue)", "\(editorKeymap.rawValue)")
        window.setEditorKeymap("\(editorKeymap.rawValue)")
        """
      )
    }
    public func setInitialContent(note: NoteModel) {
      self.runJavascript(
        """
        window.setEditorContent(\(note.markdown.body.toQuotedJavascriptString()))
        """
      )
    }
    public func setParsedEditorContentString(content: String) {
      self.runJavascript(
        """
        window.setParsedEditorContentString(\(content.toQuotedJavascriptString()))
        """)
    }
    public func emitMdxParsingError() {
      self.runJavascript(
        """
        window.emitMdxParsingError()
        """)
    }
    public func emitMdxParsingSuccess() {
      self.runJavascript(
        """
        window.emitMdxParsingSuccess()
        """)
    }
    public func setParsedEditorContent(content: Data) {
      let bytes: [UInt8] = Array(content)
      self.runJavascript(
        """
        window.setParsedEditorContent(\(bytes))
        """)
    }
    public func setInitialProperties(
      editingNote: NoteModel?,
      codeEditorTheme: CodeSyntaxTheme,
      editorKeymap: EditorKeymap,
      theme: WebViewTheme,
      fontSize: WebviewFontSize,
      editorThemeDark: CodeSyntaxTheme,
      editorThemeLight: CodeSyntaxTheme,
      darkMode: Bool,
      modelContext: ModelContext
    ) {
      self.applyWebViewColorScheme(darkMode: darkMode)
      self.emitEditorThemeEvent(theme: codeEditorTheme)
      self.setEditorKeymap(editorKeymap: editorKeymap)
      self.setWebviewTheme(theme: theme)
      self.setWebviewFontSize(fontSize: fontSize)
      self.setEditorDarkTheme(theme: editorThemeDark)
      self.setEditorLightTheme(theme: editorThemeLight)
      if let _editingNote = editingNote {
        self.setInitialContent(note: _editingNote)
        if let parsedBody = _editingNote.markdown.preParsedBody {
            print("HERERERERE")
          self.setParsedEditorContentString(content: parsedBody)
        }
      }
    }
  }
#endif
