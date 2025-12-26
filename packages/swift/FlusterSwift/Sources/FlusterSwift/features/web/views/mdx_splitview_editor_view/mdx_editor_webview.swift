import SwiftData
import SwiftUI
import WebKit

public enum CodeSyntaxTheme: String, Codable, CaseIterable {
  case materialLight, solarizedLight, githubLight, aura, tokyoNightDay,
    dracula, tokyoNight, materialDark, tokyoNightStorm, githubDark,
    solarizedDark, xcodeDark, xcodeLight
}

@MainActor
public struct MdxEditorWebviewInternal: UIViewRepresentable {
  @State private var webView: WKWebView = WKWebView(
    frame: .zero,
    configuration: getConfig()
  )
  @State private var didSetInitialContent = false
  @State var haveSetInitialContent: Bool = false
  @Environment(\.openURL) var openURL
  @Environment(\.modelContext) var modelContext
  @Environment(\.colorScheme) var colorScheme
  @Environment(\.createDataHandler) var dataHandler
  @AppStorage(AppStorageKeys.webviewFontSize.rawValue) private
    var webviewFontSize: WebviewFontSize = .base
  let url: URL
  @Binding var show: Bool
  @Binding var theme: WebViewTheme
  @Binding var editorThemeDark: CodeSyntaxTheme
  @Binding var editorThemeLight: CodeSyntaxTheme
  @Binding var editingNote: NoteModel
  @Binding var editorKeymap: EditorKeymap

  let container: MdxEditorWebviewContainer

  public init(
    url: URL,
    theme: Binding<WebViewTheme>,
    editorThemeDark: Binding<CodeSyntaxTheme>,
    editorThemeLight: Binding<CodeSyntaxTheme>,
    editingNote: Binding<NoteModel>,
    editorKeymap: Binding<EditorKeymap>,
    container: MdxEditorWebviewContainer,
    show: Binding<Bool>
  ) {
    self.url = url
    self._theme = theme
    self._editorThemeDark = editorThemeDark
    self._editorThemeLight = editorThemeLight
    self._editingNote = editingNote
    self._editorKeymap = editorKeymap
    self.container = container
    self._show = show
  }

  public func makeUIView(context: Context) -> WKWebView {
    let webView = container.webView
    webView.isHidden = true

    webView.navigationDelegate = context.coordinator
    let editorContentControllers = [
      SplitviewEditorWebviewActions.setWebviewLoaded.rawValue,
      SplitviewEditorWebviewActions.onEditorChange.rawValue,
      SplitviewEditorWebviewActions.requestSplitviewEditorData.rawValue,
      SplitviewEditorWebviewActions.requestParsedMdxContent.rawValue,
      SplitviewEditorWebviewActions.onTagClick.rawValue
    ]
    if colorScheme == .dark {
      webView.evaluateJavaScript(
        """
        document.body.classList.add("dark"); null;
        """
      )
    }
    for controllerName in editorContentControllers {
      addUserContentController(
        controller: webView.configuration.userContentController,
        coordinator: context.coordinator,
        name: controllerName
      )
    }

    // Loading the page only once
    webView.loadFileURL(url, allowingReadAccessTo: url)

    if colorScheme == .dark {
      webView.evaluateJavaScript(
        """
        document.body.classList.add("dark"); null;
        """
      )
    }

    return webView
  }

  public func updateUIView(_ uiView: WKWebView, context: Context) {
    uiView.isHidden = !show
    //        uiView.scrollView.contentInset = .zero
    //        uiView.scrollView.scrollIndicatorInsets = .zero
  }
  public func makeCoordinator() -> Coordinator {
    Coordinator(self)
  }
  public func setInitialProperties() {
    container.setInitialProperties(
      editingNote: editingNote,
      codeEditorTheme: colorScheme == .dark
        ? editorThemeDark : editorThemeLight,
      editorKeymap: editorKeymap,
      theme: theme,
      fontSize: webviewFontSize,
      editorThemeDark: editorThemeDark,
      editorThemeLight: editorThemeLight,
      darkMode: colorScheme == .dark
    )
  }
  public func setInitialContent() {
    let s = editingNote.markdown.body.toQuotedJavascriptString() ?? "''"
    container.runJavascript(
      """
      window.localStorage.setItem("\(SplitviewEditorWebviewLocalStorageKeys.initialValue.rawValue)", \(s))
      window.setEditorContent(\(s))
      """
    )
  }
}

@MainActor
extension MdxEditorWebviewInternal {
  public final class Coordinator: NSObject, WKNavigationDelegate,
    WKScriptMessageHandler
  {
    var parent: MdxEditorWebviewInternal

    init(_ parent: MdxEditorWebviewInternal) {
      self.parent = parent
    }

    public func webView(
      _ webView: WKWebView,
      didFinish navigation: WKNavigation!
    ) {
      // On Load
      guard !parent.didSetInitialContent else { return }

      webView.evaluateJavaScript(
        """
        window.localStorage.setItem("\(SplitviewEditorWebviewLocalStorageKeys.initialValue.rawValue)", \(parent.editingNote.markdown.body.toQuotedJavascriptString() ?? "''"));
        """
      )
      parent.setInitialProperties()
      parent.container.webView.isHidden = !self.parent.show
      //            parent.container.requestDocumentSize()
      parent.didSetInitialContent = true
    }

    public func webView(
      _ webView: WKWebView,
      decidePolicyFor navigationResponse: WKNavigationResponse
    ) async -> WKNavigationResponsePolicy {
      let isOriginalUrl =
        navigationResponse.response.url == self.parent.url
      if let _url = navigationResponse.response.url, !isOriginalUrl {
        self.parent.openURL(_url)
      }
      return isOriginalUrl ? .allow : .cancel
    }

    func webView(
      _ webView: WKWebView,
      decidePolicyFor navigationAction: WKNavigationAction,
      decisionHandler: @escaping (WKNavigationActionPolicy) -> Void
    ) {
      if navigationAction.navigationType == .linkActivated,
        let url = navigationAction.request.url
      {
        UIApplication.shared.open(url)
        decisionHandler(.cancel)
      } else {
        decisionHandler(.allow)
      }
    }

    public func webView(
      _ webView: WKWebView,
      didFail navigation: WKNavigation!,
      withError error: Error
    ) {
      print(
        "WebView navigation failed with error: \(error.localizedDescription)"
      )
    }
    @MainActor
    public func userContentController(
      _ userContentController: WKUserContentController,
      didReceive message: WKScriptMessage
    ) {
      switch message.name {
        case SplitviewEditorWebviewActions.setWebviewLoaded.rawValue:
          self.parent.webView.isHidden = false
          self.parent.show = true
        //                self.parent.webView.allowsMagnification
        case SplitviewEditorWebviewActions.onTagClick.rawValue:
          // TODO: Handle this tag click event.
          print("Tag clicked \(message.body as! String)")
        case SplitviewEditorWebviewActions.requestParsedMdxContent.rawValue:
          print("Received request for parsed mdx content")
          Task {
            if let parsedMdx =
              await parent.editingNote.markdown
              .body.preParseAsMdxToBytes()
            {
              parent.container.setParsedEditorContent(
                content: parsedMdx
              )
              if let parsingResults =
                parsedMdx.toMdxParsingResult()
              {
                parent.editingNote.applyMdxParsingResults(
                  results: parsingResults,
                )
              }
            }
          }
        case SharedWebviewActions.javascriptError.rawValue:
          handleJavascriptError(base64String: message.body as! String)
        case SplitviewEditorWebviewActions.onEditorChange.rawValue:
          if let str = message.body as? String {
            parent.editingNote.markdown.body = str
            parent.editingNote.setLastRead(setModified: true)
          }
        case SplitviewEditorWebviewActions.requestSplitviewEditorData
          .rawValue:
          parent.setInitialProperties()
          parent.setInitialContent()
        default:
          return
      }
    }
  }
}

public struct MdxEditorWebview: View {
  @State private var show: Bool = false
  @State private var showEditNoteTaggables: Bool = false
  @Environment(ThemeManager.self) private var themeManager: ThemeManager
  let url: URL
  @Binding var theme: WebViewTheme
  @Binding var editorThemeDark: CodeSyntaxTheme
  @Binding var editorThemeLight: CodeSyntaxTheme
  @Binding var editingNote: NoteModel
  @Binding var editorKeymap: EditorKeymap
  let container: MdxEditorWebviewContainer
  public init(
    url: URL,
    theme: Binding<WebViewTheme>,
    editorThemeDark: Binding<CodeSyntaxTheme>,
    editorThemeLight: Binding<CodeSyntaxTheme>,
    editingNote: Binding<NoteModel>,
    editorKeymap: Binding<EditorKeymap>,
    container: MdxEditorWebviewContainer,
  ) {
    self.url = url
    self._theme = theme
    self._editorThemeDark = editorThemeDark
    self._editorThemeLight = editorThemeLight
    self._editingNote = editingNote
    self._editorKeymap = editorKeymap
    self.container = container
  }

  public var body: some View {
    ZStack(alignment: show ? .bottomTrailing : .center) {
      MdxEditorWebviewInternal(
        url: url,
        theme: $theme,
        editorThemeDark: $editorThemeDark,
        editorThemeLight: $editorThemeLight,
        editingNote: $editingNote,
        editorKeymap: $editorKeymap,
        container: container,
        show: $show
      )
      .disableAnimations()
      .frame(
        alignment: .bottom
      )
      .scrollDisabled(true)
      if !show {
        ProgressView()
          .progressViewStyle(.circular)
          .scaleEffect(1.5)
          .tint(themeManager.theme.primary)
      } else {
        FloatingButtonView(
          buttons: [
            FloatingButtonItem(
              id: "addTaggable",
              systemImage: "tag.fill",
              action: {
                withAnimation {
                  showEditNoteTaggables.toggle()
                }
              }
            ),
            FloatingButtonItem(
              id: "toggleBookmarked",
              systemImage: editingNote.bookmarked ? "bookmark.fill" : "bookmark",
              action: {
                editingNote.bookmarked.toggle()
              }
            )
          ]
        )
        .padding()
      }
    }
    .fullScreenCover(
      isPresented: $showEditNoteTaggables,
      content: {
        EditNoteTaggablesView(
          editingNote: $editingNote,
          open: $showEditNoteTaggables
        )
      },
    )
  }
}
