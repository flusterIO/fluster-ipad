//
//  File.swift
//  FlusterSwift
//
//  Created by Andrew on 12/5/25.
//

import SwiftData
import SwiftUI
import WebKit
import FlusterData


#if os(iOS)
public struct NoteDetailWebviewInternal: UIViewRepresentable {
  @State private var lastNoteId: String? = nil
  @State private var didSetInitialContent: Bool = false
  @Environment(\.colorScheme) private var colorScheme: ColorScheme
  @Environment(\.modelContext) private var modelContext: ModelContext
  @AppStorage(AppStorageKeys.theme.rawValue) private var webviewTheme: WebViewTheme =
    .fluster
  @AppStorage(AppStorageKeys.webviewFontSize.rawValue) private
    var webviewFontSize: WebviewFontSize = .base
  public let url: URL = Bundle.main.url(
    forResource: "index",
    withExtension: "html",
    subdirectory: "note_detail_webview"
  )!

  //    @Binding public var tagQuery: String
  @Binding public var fullScreenCover: MainFullScreenCover?
  @Binding public var show: Bool
  @Binding public var note: NoteModel

  public let container: NoteDetailWebviewContainer

  public init(
    fullScreenCover: Binding<MainFullScreenCover?>,
    note: Binding<NoteModel>,
    show: Binding<Bool>,
    //        tagQuery: Binding<String>,
    container: NoteDetailWebviewContainer
  ) {
    self._fullScreenCover = fullScreenCover
    self._note = note
    self._show = show
    //        self._tagQuery = tagQuery
    self.container = container
  }

  public func handleTagClick(tagBody: String) {
    let fetchDescriptor = FetchDescriptor<TagModel>(
      predicate: #Predicate<TagModel> { t in
        t.value == tagBody
      }
    )
    do {
      let res = try modelContext.fetch(fetchDescriptor)
      FlusterLogger(.webview, .devOnly).log("Res: \(res)", .info)
      if !res.isEmpty {
        self.fullScreenCover = .tagSearch(tag: res.first!)
      }
    } catch {
      FlusterLogger(.mainApp, .prodAndDev).log(
        "Failed to find tag in handleTagClick: \(error)", .warning)
    }
  }
  func handleSubjectClick(subjectValue: String) {
    let fetchDescriptor = FetchDescriptor<SubjectModel>(
      predicate: #Predicate<SubjectModel> { t in
        t.value == subjectValue
      }
    )
    do {
      let res = try modelContext.fetch(fetchDescriptor)
      if !res.isEmpty {
        self.fullScreenCover = .subjectSearch(subject: res.first!)
      }
    } catch {
      FlusterLogger(.mainApp, .prodAndDev).log(
        "Failed to find subject in handleSubjectClick: \(error)", .warning)
    }
  }

  func handleTopicClck(topicValue: String) {
    let fetchDescriptor = FetchDescriptor<TopicModel>(
      predicate: #Predicate<TopicModel> { t in
        t.value == topicValue
      }
    )
    do {
      let res = try modelContext.fetch(fetchDescriptor)
      if !res.isEmpty {
        self.fullScreenCover = .topicSearch(topic: res.first!)
      }
    } catch {
      FlusterLogger(.mainApp, .prodAndDev).log(
        "Failed to find topic in handleTopicClick: \(error)", .warning)
    }
  }

  public func makeUIView(context: Context) -> WKWebView {
    let webView = container.webView

    webView.navigationDelegate = context.coordinator
    webView.isHidden = true
    let editorContentControllers = [
      NoteDetailWebviewActions.requestNoteDetailData.rawValue,
      NoteDetailWebviewActions.setWebviewLoaded.rawValue,
      NoteDetailWebviewActions.handleTagClick.rawValue,
      NoteDetailWebviewActions.handleTopicClick.rawValue,
      NoteDetailWebviewActions.handleSubjectClick.rawValue,
      NoteDetailWebviewActions.handleCitationClick.rawValue
    ]
    for controllerName in editorContentControllers {
      addUserContentController(
        controller: webView.configuration.userContentController,
        coordinator: context.coordinator,
        name: controllerName
      )
    }

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
  }
  public func makeCoordinator() -> Coordinator {
    Coordinator(self)
  }
  public func handleCitationClick(citationId: String) {
    let descriptor = FetchDescriptor<BibEntryModel>(
      predicate: #Predicate<BibEntryModel> { entry in
        entry.citationKey == citationId
      })
    if let res = try? self.modelContext.fetch(descriptor) {
      print("Here... ", res)
      let entry = res.first!
      self.fullScreenCover = .citationByKey(citation: entry)
    }
  }
}

@MainActor
extension NoteDetailWebviewInternal {
  public final class Coordinator: NSObject, WKNavigationDelegate,
    WKScriptMessageHandler
  {
    var parent: NoteDetailWebviewInternal

    public init(_ parent: NoteDetailWebviewInternal) {
      self.parent = parent
    }

    public func webView(
      _ webView: WKWebView,
      didFinish navigation: WKNavigation!
    ) {
      // On Load
      guard !parent.didSetInitialContent else { return }
      parent.container.webView.isHidden = !self.parent.show
      parent.didSetInitialContent = true
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

    public func setInitialData() {
      self.parent.container.setInitialData(
        colorScheme: self.parent.colorScheme,
        webviewTheme: self.parent.webviewTheme,
        fontSize: self.parent.webviewFontSize,
        note: self.parent.note
      )
    }

    @MainActor
    public func userContentController(
      _ userContentController: WKUserContentController,
      didReceive message: WKScriptMessage
    ) {
      switch message.name {
        case NoteDetailWebviewActions.setWebviewLoaded.rawValue:
          self.parent.show = true
        case SharedWebviewActions.javascriptError.rawValue:
          handleJavascriptError(base64String: message.body as! String)
        case NoteDetailWebviewActions.requestNoteDetailData.rawValue:
          self.setInitialData()
        case NoteDetailWebviewActions.handleTagClick.rawValue:
          self.parent.handleTagClick(tagBody: message.body as! String)
        case NoteDetailWebviewActions.handleTopicClick.rawValue:
          self.parent.handleTopicClck(topicValue: message.body as! String)
        case NoteDetailWebviewActions.handleSubjectClick.rawValue:
          self.parent.handleSubjectClick(subjectValue: message.body as! String)
        case NoteDetailWebviewActions.handleCitationClick.rawValue:
          self.parent.handleCitationClick(citationId: message.body as! String)
        default:
          return
      }
    }
  }
}

public struct NoteDetailWebview: View {
  @Environment(\.colorScheme) private var colorScheme: ColorScheme
  @Environment(ThemeManager.self) private var themeManager: ThemeManager
  @AppStorage(AppStorageKeys.theme.rawValue) private var webviewTheme: WebViewTheme =
    .fluster
  @AppStorage(AppStorageKeys.webviewFontSize.rawValue) private
    var webviewFontSize: WebviewFontSize = .base
  @State private var show: Bool = false
  @State var container: NoteDetailWebviewContainer =
    NoteDetailWebviewContainer(
      bounce: true,
      scrollEnabled: true,
      onLoad: nil
    )

  @Binding var fullScreenCover: MainFullScreenCover?
  @Binding var note: NoteModel

  public init(
    fullScreenCover: Binding<MainFullScreenCover?>,
    note: Binding<NoteModel>
  ) {
    self._fullScreenCover = fullScreenCover
    self._note = note
  }
  public var body: some View {
    ZStack {
      NoteDetailWebviewInternal(
        fullScreenCover: $fullScreenCover,
        note: $note,
        show: $show,
        container: container
      )
      .onAppear {
        self.container.setInitialData(
          colorScheme: self.colorScheme,
          webviewTheme: self.webviewTheme,
          fontSize: self.webviewFontSize,
          note: self.note
        )
      }
      if !show {
        ProgressView()
          .progressViewStyle(.circular)
          .scaleEffect(1.5)
          .tint(themeManager.theme.primary)
      }
    }
    .background(colorScheme == .dark ? .black : .white)
    .onChange(
      of: note,
      {
        if self.show {
          container.setNoteDetails(note: note)
        }
      }
    )
    .onChange(
      of: note.markdown.body,
      {
        if self.show {
          container.setNoteDetails(note: note)
        }
      }
    )
    .onChange(
      of: colorScheme,
      {
        if self.show {
          container.applyWebViewColorScheme(
            darkMode: colorScheme == .dark
          )
        }
      }
    )
    .onChange(
      of: webviewTheme,
      {
        if self.show {
          container.setWebviewTheme(theme: webviewTheme)
        }
      }
    )
    .onChange(
      of: webviewFontSize,
      {
        if self.show {
          container.setWebviewFontSize(fontSize: webviewFontSize)
        }
      }
    )
  }
}
#endif
