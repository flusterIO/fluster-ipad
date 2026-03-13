import Combine
import FlusterData
import FlusterWebviewClients
import SwiftData
import SwiftUI
import WebKit

#if os(iOS)
  @MainActor func getConfig() -> WKWebViewConfiguration {
    let config = WebContextManager.createSharedConfiguration()
    config.setURLSchemeHandler(WasmSchemeHandler(), forURLScheme: "app")
    config.setValue(true, forKey: "allowUniversalAccessFromFileURLs")
    config.preferences.setValue(true, forKey: "allowFileAccessFromFileURLs")
    //    config.preferences.setValue(true, forKey: "")
    return config
  }

  //typealias AnyWebviewAction = NoteDetailWebviewActions | SplitviewEditorWebviewActions
  @MainActor
  public class WebviewContainer:
    NSObject, ObservableObject, WKNavigationDelegate {
    public let scrollViewBounce: Bool = true
    public let scrollEnabled: Bool = false
    public var onLoad: (@Sendable (WKWebView) async -> Void)?
    @Binding private var editingNote: NoteModel?
    public var implementation: WebviewImplementation
    @Environment(\.modelContext) private var modelContext: ModelContext
    @Query(sort: \BibEntryModel.citationKey) private var bibEntries: [BibEntryModel]
    @Environment(\.colorScheme) private var colorScheme: ColorScheme

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
    lazy var webView: WKWebView = {
      let config = getConfig()
      let view = WKWebView(frame: .zero, configuration: config)
      view.isHidden = true
      view.scrollView.bouncesZoom = false  // Disable zoom bounce
      view.scrollView.minimumZoomScale = 1.0  // Prevent zooming out
      view.scrollView.maximumZoomScale = 1.0
      let source = """
        document.body?.classList.add("\(WebviewEnvironment.iPad.rawValue)")
        """
      view.allowsBackForwardNavigationGestures = false
      let webviewEnvironmentScript = WKUserScript(
        source: source,
        injectionTime: .atDocumentEnd,
        forMainFrameOnly: true
      )

      view.configuration.userContentController.addUserScript(webviewEnvironmentScript)
      view.navigationDelegate = self
      if #available(iOS 16.4, macOS 13.3, *) {
        view.isInspectable = true
      }

      return view
    }()  // Required to be iffe instead of computed property to maintain state.

    public init(
      bounce: Bool = false,
      scrollEnabled: Bool = false,
      onLoad: (@Sendable (WKWebView) async -> Void)?,
      editingNote: Binding<NoteModel?>,
      implementation: WebviewImplementation
    ) {
      self.onLoad = onLoad
      self._editingNote = editingNote
      self.implementation = implementation
      super.init()
      self.webView.scrollView.bounces = bounce
      self.webView.scrollView.isScrollEnabled = scrollEnabled
    }

    public func runJavascript(_ script: String) {
      self.webView.evaluateJavaScript(script) { result, error in
        if let error = error {
          print("--- Error ---")
          print("Error executing JS: \(error.localizedDescription)")
          print("Command: \(script.trunc(length: 200))")
          print("-------------")
        } else if let result = result {
          print("JS Result: \(result)")
        }
      }
    }

    /// A utility function used to append some initial styles to the window before loading the webview. Not sure if this will even work...
    public func preShow(colorScheme: ColorScheme) {
      self.runJavascript(
        """
        document.body.classList.add('\(colorScheme == .dark ? "dark" : "light")'); null;
        """
      )
    }

    public func setWebviewTheme(theme: FlusterTheme) {
      Task(priority: .high) {
        do {
          try await WebviewContainerState.setFlusterTheme(
            payload: SetFlusterThemePayload(fluster_theme: theme),
            eval: self.webView.evaluateJavaScript)
        } catch {
          print("Error: \(error.localizedDescription)")
        }
      }
    }
    public func setWebviewFontSize(fontSize: WebviewFontSize) {
      self.runJavascript(
        """
        window.setWebViewFontSize('\(fontSize.cssClass())'); null;
        """
      )
    }
    public func applyWebViewColorScheme(colorScheme: ColorScheme) {
      Task(priority: .high) {
        do {
          try await EditorState.setDarkMode(
            colorScheme: colorScheme, eval: self.webView.evaluateJavaScript)
        } catch {
          print("Error: \(error.localizedDescription)")
        }
      }
    }
    public func setEditorKeymap(keymap: CodeEditorKeymap) {
      Task(priority: .high) {
        do {
          try await EditorState.setEditorKeymap(
            keymap: keymap, eval: self.webView.evaluateJavaScript)
        } catch {
          print("Error: \(error.localizedDescription)")
        }
      }
    }

    public func setEditorThemeDark(theme: CodeEditorTheme) {
      Task(priority: .high) {
        do {
          try await EditorState.setEditorThemeDark(
            payload: SetEditorThemeDarkPayload(theme_dark: theme),
            eval: self.webView.evaluateJavaScript)
        } catch {
          print("Error: \(error.localizedDescription)")
        }
      }
    }
    public func setEditorThemeLight(theme: CodeEditorTheme) {
      Task(priority: .high) {
        do {
          try await EditorState.setEditorThemeLight(
            payload: SetEditorThemeLightPayload(theme_light: theme),
            eval: self.webView.evaluateJavaScript)
        } catch {
          print("Error: \(error.localizedDescription)")
        }
      }
    }
    public func resetScrollPosition(
      containerId: String = "mdx-preview",
      scrollStorageKeys: [String]
    ) {
      var s = ""
      for (i, scrollKey) in scrollStorageKeys.enumerated() {
        s = s + "\"\(scrollKey)\"\(i == scrollStorageKeys.count - 1 ? "" : ",")"
      }
      self.runJavascript(
        """
        window.resetMdxPreviewScrollPosition("\(containerId)", [\(s)])
        """)
    }

    public func requestDocumentSize() {
      self.runJavascript(
        """
        window.requestDocumentSize(); null;
        """
      )
    }
    public func addClassToWebviewContainer(className: String) {
      Task(priority: .high) {
        do {
          try await WebviewContainerClient.setClassToWebviewContainer(
            className: className, method: .append, evaluateJavaScript: webView.evaluateJavaScript)
        } catch {
          print("Error: \(error.localizedDescription)")
        }
      }
    }
    public func removeClassFromWebviewContainer(className: String) {
      Task(priority: .high) {
        do {
          try await WebviewContainerClient.setClassToWebviewContainer(
            className: className, method: .remove, evaluateJavaScript: webView.evaluateJavaScript)
        } catch {
          print("Error: \(error.localizedDescription)")
        }
      }
    }

    public func setLoading(isLoading: Bool) {
      if isLoading {
        self.addClassToWebviewContainer(className: "loading")
      } else {
        self.removeClassFromWebviewContainer(className: "loading")
      }
    }

    public func sendScreenSize() {
      let width = UIScreen.current?.bounds.width
      let height = UIScreen.current?.bounds.height
      if width != nil && height != nil {
        self.runJavascript(
          """
          window.setScreenSize(\(width), \(height)); null;
          """
        )
      }
    }
    // Redux State Management
    public func setInitialState() async throws {
      if let en = editingNote {
        try await en.preParseIfEdited(modelContext: modelContext)
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
            environment: WebviewEnvironment.macOS,
            dark_mode: colorScheme == .dark,
            implementation: self.implementation,
            fluster_theme: flusterTheme
          ),
          eval: self.webView.evaluateJavaScript
        )
      }
    }

    // Delegate Methods

    // onLoad
    public func webView(_ webView: WKWebView, didFinish navigation: WKNavigation!) {
      setLoading(isLoading: false)
      if let ol = self.onLoad {
        Task(priority: .high) {
          do {
            try await self.setInitialState()
          } catch {
            print("Error: \(error.localizedDescription)")
          }
          await ol(webView)
        }
      }
    }
  }
#endif
