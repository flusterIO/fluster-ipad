//
//  webview_container.swift
//  Fluster
//
//  Created by Andrew on 1/20/26.
//

import FlusterAI
import FlusterData
import FlusterWebviewClients
import FoundationModels
import SwiftData
import SwiftUI
import WebKit

func getWebViewConfig() -> WKWebViewConfiguration {
  let config = WebContextManager.createSharedConfiguration()
  config.setURLSchemeHandler(WasmSchemeHandler(), forURLScheme: "app")
  config.setValue(true, forKey: "allowUniversalAccessFromFileURLs")
  config.preferences.setValue(true, forKey: "allowFileAccessFromFileURLs")
  return config
}

struct WebViewContainer: NSViewRepresentable {  // Use UIViewRepresentable for iOS/iPadOS
  let parent: WebViewContainerView
  @Binding var webView: WKWebView
  @Environment(\.colorScheme) private var colorScheme: ColorScheme
  @AppStorage(AppStorageKeys.userPreferredName.rawValue) private var userPreferedName: String = ""
  let url: URL
  var messageHandlerKeys: [String]
  var messageHandler: ((String, Any) -> Void)?
  var onLoad: (@Sendable () async -> Void)?

  typealias ViewType = WKWebView
  func makeNSView(context: Context) -> WKWebView { makeWebView(context: context) }
  func updateNSView(_ nsView: WKWebView, context: Context) {
  }

  private func makeWebView(context: Context) -> WKWebView {
    webView.isHidden = true
    webView.setValue(false, forKey: "drawsBackground")
    webView.underPageBackgroundColor = .clear
    webView.wantsLayer = true
    webView.layer?.backgroundColor = NSColor.windowBackgroundColor.cgColor
    if colorScheme == .dark {
      addDarkModeScript(controller: webView.configuration.userContentController)
    }
    var all_keys = [
      WebviewContainerEvents.reduxStateLoaded.rawValue,
      AiStateEvents.sendGeneralAiRequestPhase2.rawValue,
      NoteDetailEvents.sendGenerateSummaryRequest.rawValue
    ]
    for mk in messageHandlerKeys {
      all_keys.append(mk)
    }
    addContentControllerList(
      items: all_keys,
      controller: webView.configuration.userContentController,
      coordinator: context.coordinator
    )

    DispatchQueue.main.async {
      if let scrollView = webView.enclosingScrollView {
        scrollView.horizontalScrollElasticity = .none
        scrollView.verticalScrollElasticity = .none
        scrollView.showContextHelp(false)
      }
    }
    webView.navigationDelegate = context.coordinator

    webView.isInspectable = true
    let source = """
      document.body?.classList.add("\(WebviewEnvironment.macOS.rawValue)")
      """
    let webviewEnvironmentScript = WKUserScript(
      source: source,
      injectionTime: .atDocumentEnd,
      forMainFrameOnly: true
    )
    webView.configuration.userContentController.addUserScript(webviewEnvironmentScript)

    //    webView.loadFileURL(url, allowingReadAccessTo: url.deletingLastPathComponent())
    let request = URLRequest(url: url)
    webView.load(request)
    return webView
  }

  func addDarkModeScript(controller: WKUserContentController) {
    let source = """
      document.body?.classList.add("dark")
      """

    // 2. Create the script to run at the very start (atDocumentStart)
    let userScript = WKUserScript(
      source: source,
      injectionTime: .atDocumentEnd,
      forMainFrameOnly: true
    )
    controller.addUserScript(userScript)
  }

  func handleNoteSummaryCreation(msg: String) {
    print("Creating summary...")
    do {
      if let en = self.parent.editingNote {
        if let jsonData = msg.data(using: .utf8) {
          let decoder = JSONDecoder()
          let event = try decoder.decode(GenerateNoteSummaryRequest.self, from: jsonData)
          if event.note_id == en.id {
            Task {
              let res = try await FlusterAI.generateNoteSummary(
                focusedNote: en, details: AIUserDetails(preferred_name: userPreferedName))
              print("Res: \(res)")
              if res.success {
                en.frontMatter.applyAIGeneratedSummary(
                  content: res.res,
                  generationMethod: NoteSummaryGenerationMethod.fromSummaryGenerationMethod(
                    event.generation_method))
                if let summaryState = en.frontMatter.summary?.toSummaryState() {
                  Task {
                    do {
                      try await NoteDetailState.setNoteSummary(
                        payload: summaryState, eval: webView.evaluateJavaScript)
                    } catch {
                      print("Error: \(error.localizedDescription)")
                    }
                  }
                }
              }
            }
          }
        }
      }
    } catch {
      print("Error: \(error.localizedDescription)")
    }
  }

  func makeCoordinator() -> Coordinator {
    Coordinator(self)
  }

  class Coordinator: NSObject, WKScriptMessageHandler, WKNavigationDelegate {
    let parent: WebViewContainer
    init(_ parent: WebViewContainer) {
      self.parent = parent
    }

    // 2. This function catches the JS "postMessage"
    func userContentController(
      _ userContentController: WKUserContentController, didReceive message: WKScriptMessage
    ) {
      if message.name == WebviewContainerEvents.reduxStateLoaded.rawValue {
        Task(priority: .high) {
          await self.parent.parent.handleInitialState()
          if let onLoad = parent.onLoad {
            await onLoad()
          }
        }
      } else if message.name == NoteDetailEvents.sendGenerateSummaryRequest.rawValue {
        self.parent.handleNoteSummaryCreation(msg: message.body as! String)
      } else if message.name == AiStateEvents.sendGeneralAiRequestPhase2.rawValue {
        if let jsonData = (message.body as! String).data(using: .utf8) {
          do {
            let decoder = JSONDecoder()
            let event = try decoder.decode(GeneralAiRequestPhase2Event.self, from: jsonData)
            // WITH_WIFI: Figure out how to handle this error here. It works, but this should definitely be handled.
            // Task(priority: .high) {
            //              let res = handleGeneralMdxAiRequest(
            //                request: event, focusedNote: parent.parent.editingNote)
            // RESUME: Get the response here. If there is data to be replaced call a function in Rust to get the new content and replace it. If there is a user notification message, send a notification to the user.
            // }
          } catch {
            print("Failed to decode editor update: \(error)")
          }
        }
      }
      if let messageHandler = parent.messageHandler {
        for handler in parent.messageHandlerKeys {
          if message.name == handler {
            messageHandler(handler, message.body)
          }
        }
      } else {
        print("No message handler found.")
        return
      }
    }
    func webView(
      _ webView: WKWebView,
      decidePolicyFor navigationAction: WKNavigationAction,
      decisionHandler: @escaping (WKNavigationActionPolicy) -> Void
    ) {
      // Check if the navigation was triggered by a user clicking a link
      if navigationAction.navigationType == .linkActivated {
        if let url = navigationAction.request.url {
          // Open the URL in the system default browser
          NSWorkspace.shared.open(url)

          // Cancel the navigation within the webview
          decisionHandler(.cancel)
          return
        }
      }

      // Allow other types of navigation (like the initial load)
      decisionHandler(.allow)
    }
    func webView(
      _ webView: WKWebView, didFailProvisionalNavigation navigation: WKNavigation!,
      withError error: Error
    ) {
      print("Navigation failed early: \(error.localizedDescription)")
    }
    func webView(_ webView: WKWebView, didFinish navigation: WKNavigation!) {
      print("Web content loaded...")
      // Removing this in favor of redux based approach.
      // This will break if all webviews don't use the same
      // Redux container.
      //      if let onLoad = parent.onLoad {
      //        Task(priority: .high) {
      //          await onLoad()
      //        }
      //      }
      parent.webView.isHidden = false
    }
  }
}

struct WebViewContainerView: View {
  let implementation: WebviewImplementation
  @EnvironmentObject private var appState: AppState
  @Environment(\.modelContext) private var modelContext: ModelContext

  @Query(sort: \BibEntryModel.citationKey) private var bibEntries: [BibEntryModel]
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
  @AppStorage(AppStorageKeys.storePlainText.rawValue) private var storePlainText: Bool = true
  @AppStorage(AppStorageKeys.includeEmojiSnippets.rawValue) private var includeEmojiSnippets: Bool =
    true
  @Binding var webview: WKWebView
  public let url: URL
  public let messageHandlerKeys: [String]
  public let messageHandler: ((String, Any) -> Void)?
  public let onLoad: (@Sendable () async -> Void)?
  public let mathjaxFontUrl: String

  @Query private var notes: [NoteModel]

  var editingNote: NoteModel? {
    notes.isEmpty ? nil : notes.first!
  }

  @State private var show: Bool = false
  @AppStorage(AppStorageKeys.theme.rawValue) private var theme: FlusterTheme = .fluster
  @Environment(\.colorScheme) private var colorScheme: ColorScheme

  init(
    implementation: WebviewImplementation,
    editingNoteId: String?,
    webview: Binding<WKWebView>,
    url: URL,
    messageHandlerKeys: [String],
    messageHandler: ((String, Any) -> Void)?,
    onLoad: (@Sendable () async -> Void)?,
    mathjaxFontUrl: String,
  ) {
    self.implementation = implementation
    self.mathjaxFontUrl = mathjaxFontUrl
    self._webview = webview
    self.url = url
    self.messageHandlerKeys = messageHandlerKeys
    self.messageHandler = messageHandler
    self.onLoad = onLoad
    if let _editingNoteId = editingNoteId {
      var descriptor = FetchDescriptor(
        predicate: #Predicate<NoteModel> { note in
          note.id == _editingNoteId
        }
      )
      descriptor.fetchLimit = 1
      self._notes = Query(
        descriptor
      )
    } else {
      self._notes = Query(
        FetchDescriptor(
          predicate: #Predicate<NoteModel> { _ in
            false
          }
        ))
    }
  }

  var body: some View {
    ZStack {
      WebViewContainer(
        parent: self,
        webView: $webview,
        url: url,
        messageHandlerKeys: messageHandlerKeys,
        messageHandler: messageHandler,
        onLoad: onLoadHandler
      )
      if !show {
        ProgressView("Loading...")
          .tint(Color.accent)
          .foregroundStyle(.foreground)
          .progressViewStyle(.circular)
      }
    }
    .task(priority: .high) {
      await handleInitialState()
    }
    // WITH_WIFI: Figure out how to move this back to a background thread _immediately._ It's causing a memory issue right now but that's crucial for performance, as the plain text results are only useful for search related features.
    .task(id: editingNote?.markdown._body, priority: .low) {
      if let en = editingNote, storePlainText {
        do {
          try await en.markdown.parsePlainText(noteId: en.id)
        } catch {
          print("Error parsing plain text: \(error.localizedDescription)")
        }
      }
    }
    .onChange(
      of: editingNote?.id,
      {
        Task(priority: .high) {
          await handleInitialState()
        }
      }
    )
    .onChange(
      of: editingNote?.markdown.preParsedBody,
      {
        if editorSaveMethod == .onChange {
          updateParsedEditorValue()
        }
      }
    )
    .onChange(
      of: editorThemeDark,
      {
        do {
          Task(
            priority: .high,
            operation: {
              try await self.setEditorThemeDark(editorTheme: editorThemeDark)
            })
        }
      }
    )
    .onChange(
      of: editorThemeLight,
      {
        do {
          Task(
            priority: .high,
            operation: {
              try await self.setEditorThemeLight(editorTheme: editorThemeLight)
            })
        }
      }
    )
    .onChange(
      of: colorScheme,
      {
        Task {
          await setColorScheme(colorScheme: colorScheme)
        }
      }
    )
    .scrollBounceBehavior(.basedOnSize, axes: [])
  }
  public func handleInitialState() async {
    if let en = editingNote {
      Task {
        let llm = SystemLanguageModel()
        do {
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
              saveMethod: editorSaveMethod,
            ),
            containerPayload: WebviewContainerSharedInitialState(
              environment: WebviewEnvironment.macOS,
              dark_mode: colorScheme == .dark,
              implementation: self.implementation,
              fluster_theme: flusterTheme
            ),
            mathPayload: InitialMathState(mathjax_font_url: mathjaxFontUrl),
            aiPayload: AiInitialStatePayload(
              foundation_model_access: llm.availability.toReduxRepresentation()
            ),
            eval: self.webview.evaluateJavaScript
          )
        } catch {
          print("Error initializing Mdx Editor Webview: \(error.localizedDescription)")
        }
      }
    }
  }
  func onLoadHandler() async {
    await handleInitialState()
    if let additionalOnLoad = onLoad {
      await additionalOnLoad()
    }
    show = true
  }
  func runJavascript(_ js: String) async throws {
    do {
      try await webview.evaluateJavaScript(js)
    } catch {
      print("Error evaluating javascript: \(error.localizedDescription)")
    }
  }
  func setColorScheme(colorScheme: ColorScheme) async {
    do {
      try await EditorState.setDarkMode(colorScheme: colorScheme, eval: webview.evaluateJavaScript)
    } catch {
      print("Error: \(error.localizedDescription)")
    }
  }
  func updateParsedEditorValue() {
    if let en = editingNote {
      Task(priority: .high) {
        try? await en.preParseIfEdited(modelContext: modelContext)
        let citations: [EditorCitation] = en.citations.compactMap { cit in
          cit.toEditorCitation(activeCslFile: cslFile)
        }
        try? await EditorState.setParsedMdxContent(
          parsedMdxContent: en.markdown.preParsedBody ?? "", citations: citations,
          eval: webview.evaluateJavaScript)
      }
    }
  }

  func setEditorThemeDark(editorTheme: CodeEditorTheme) async throws {
    try await EditorState.setEditorThemeDark(
      payload: SetEditorThemeDarkPayload(theme_dark: editorTheme),
      eval: self.webview.evaluateJavaScript
    )
  }
  func setEditorThemeLight(editorTheme: CodeEditorTheme) async throws {
    try await EditorState.setEditorThemeLight(
      payload: SetEditorThemeLightPayload(theme_light: editorTheme),
      eval: self.webview.evaluateJavaScript
    )
  }
}
