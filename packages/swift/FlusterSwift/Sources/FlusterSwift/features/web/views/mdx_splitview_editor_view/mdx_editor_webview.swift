import FlusterData
import SwiftData
import SwiftUI
import WebKit

#if os(iOS)
  public enum MdxViewFullScreenCover {
    case byTag(tagModel: BibEntryModel)
  }

  // NOTE: Moved to FlusterData. Delete once sure this works.
  // public enum CodeSyntaxTheme: String, Codable, CaseIterable {
  //   case materialLight, solarizedLight, githubLight, aura, tokyoNightDay,
  //     dracula, tokyoNight, materialDark, tokyoNightStorm, githubDark,
  //     solarizedDark, xcodeDark, xcodeLight
  // }

  public struct MdxEditorWebview: View {
    @State private var show: Bool = false
    let url: URL
    @Binding var editingNote: NoteModel
    @Environment(\.modelContext) private var modelContext: ModelContext
    @Environment(\.colorScheme) private var colorScheme: ColorScheme
    @Binding var fullScreenCover: MainFullScreenCover?
    @State private var showEditNoteTaggables: Bool = false
    @AppStorage(AppStorageKeys.embeddedCslFile.rawValue) private var cslFile: EmbeddedCslFileSwift =
      .apa

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
    @AppStorage(AppStorageKeys.theme.rawValue) private var flusterTheme: FlusterTheme = .fluster
    @AppStorage(AppStorageKeys.includeEmojiSnippets.rawValue) private var includeEmojiSnippets:
      Bool =
        true

    var onNavigateToNote: (NoteModel) -> Void
    @State private var webView: WKWebView = WKWebView(
      frame: CGRect(x: 0, y: 0, width: 1, height: 1), configuration: getConfig()
    )

    public init(
      editingNote: Binding<NoteModel>,
      fullScreenCover: Binding<MainFullScreenCover?>,
      onNavigateToNote: @escaping (NoteModel) -> Void,
    ) {
      self.url = URL.embeddedFlusterUrl(
        folder: "splitview_mdx_editor_ipad", fileName: "index_ipad.html")
      self._editingNote = editingNote
      self._fullScreenCover = fullScreenCover
      self.onNavigateToNote = onNavigateToNote
    }

    public var body: some View {
      ZStack(alignment: show ? .bottomTrailing : .center) {
        IosWebviewContainer(
          implementation: WebviewImplementation.mdxEditor,
          editingNote: Binding<NoteModel>.toOptional($editingNote),
          webView: $webView,
          url: url,
          messageHandlerKeys: [
            MdxPreviewWebviewActions.onTagClick.rawValue,
            MdxPreviewWebviewActions.requestNoteData.rawValue,
            SplitviewEditorWebviewActions.onEditorChange.rawValue,
            SplitviewEditorWebviewActions.setWebviewLoaded.rawValue,
            SplitviewEditorWebviewActions.requestSplitviewEditorData.rawValue,
            SplitviewEditorWebviewActions.manualSaveRequest.rawValue
          ],
          messageHandler: self.messageHandler,
          onLoad: {
            show = true
          }
        )
        .frame(maxWidth: .infinity, maxHeight: .infinity)
        .disableAnimations()
        .ignoresSafeArea(edges: .bottom)
        .scrollDisabled(true)
        if !show {
          ProgressView()
            .progressViewStyle(.circular)
            .scaleEffect(1.5)
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
      .task(
        id: editingNote.markdown._body, priority: .userInitiated,
        {
          // Parse editing note body every time body is changed.
          // Doing things this way will make the task automatically cancellable
          // when the next change event is fired.
          do {
            try await editingNote.preParse(modelContext: modelContext)
          } catch {
            print("Error: \(error.localizedDescription)")
          }
        }
      )

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

    func messageHandler(_ messageKey: String, _ messageBody: Any) {
      print("Message up here: \(messageKey)")
      switch messageKey {
        case SplitviewEditorWebviewActions.requestSplitviewEditorData.rawValue:
          Task(priority: .high) {
            await self.handleInitialState()
          }
        case SplitviewEditorWebviewActions.onEditorChange.rawValue:
          if let jsonData = (messageBody as! String).data(using: .utf8) {
            do {
              let decoder = JSONDecoder()
              let event = try decoder.decode(EditorChangeEvent.self, from: jsonData)
              handleEditorChange(event: event)
            } catch {
              print("Failed to decode editor update: \(error)")
            }
          }
        case SplitviewEditorWebviewActions.manualSaveRequest.rawValue:
          Task(priority: .userInitiated) {
            await self.handleManualSaveRequest(messageBody: messageBody as! String)
          }
        default:
          return
      }
    }

    func handleManualSaveRequest(messageBody: String) async {
      if let jsonData = messageBody.data(using: .utf8) {
        do {
          let decoder = JSONDecoder()
          let event = try decoder.decode(ManualSaveRequestEvent.self, from: jsonData)
          if event.note_id == editingNote.id {
            editingNote.markdown.body = event.current_note_content
            try await editingNote.preParse(modelContext: modelContext)
            let citations = editingNote.citations.compactMap { cit in
              cit.toEditorCitation(activeCslFile: cslFile)
            }
            try await EditorState.setParsedMdxContent(
              parsedMdxContent: editingNote.markdown.preParsedBody ?? "", citations: citations,
              eval: webView.evaluateJavaScript)
          } else {
            print("Broken state: Found mismatched note id's")
          }
        } catch {
          print("Failed to decode editor update: \(error)")
        }
      }
    }
    public func handleInitialState() async {
      Task {
        do {
          try await editingNote.preParse(modelContext: modelContext)
          try await EditorState.setInitialEditorState(
            editorPayload: EditorInitialStatePayload(
              note_id: editingNote.id,
              keymap: editorKeymap,
              theme_light: editorThemeLight,
              theme_dark: editorThemeDark,
              allCitationIds: bibEntries.compactMap(\.citationKey),
              value: editingNote.markdown.body,
              parsedValue: editingNote.markdown.preParsedBody ?? "",
              haveSetInitialValue: true,
              snippetProps: SnippetState(
                includeEmojiSnippets: includeEmojiSnippets
              ),
              lockEditorScrollToPreview: lockEditorScrollToPreview,
              saveMethod: editorSaveMethod
            ),
            containerPayload: WebviewContainerSharedInitialState(
              environment: WebviewEnvironment.iPad,
              dark_mode: colorScheme == .dark,
              implementation: WebviewImplementation.mdxEditor,
              fluster_theme: flusterTheme
            ),
            eval: self.webView.evaluateJavaScript
          )
        } catch {
          print("Error initializing Mdx Editor Webview: \(error.localizedDescription)")
        }
      }
    }

    public func handleEditorChange(event: EditorChangeEvent) {
      // Don't worry about actually parsing the data. That's all
      // being handled by async tasks managed by SwiftUI for better
      // cancellation management, since it's running onChange.
      if editingNote.id == event.note_id {
        editingNote.markdown.body = event.content
        editingNote.setLastRead(setModified: true)
      }
    }

    func onLoad() async {
    }
  }
#endif
