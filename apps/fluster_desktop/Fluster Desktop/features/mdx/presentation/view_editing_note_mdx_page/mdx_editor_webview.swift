//
//  mdx_editor_webview.swift
//  Fluster
//
//  Created by Andrew on 1/20/26.
//

import FlatBuffers
import FlusterData
import FlusterMdx
import FlusterSwift
import FlusterWebviewClients
import SwiftData
import SwiftUI
import WebKit

struct MdxEditorWebview: View {
  var editingNoteId: String?
  @Environment(\.colorScheme) public var colorScheme
  @EnvironmentObject private var appState: AppState
  @Query private var notes: [NoteModel]
  @State private var showDeleteConfirmation: Bool = false

  var editingNote: NoteModel? {
    notes.isEmpty ? nil : notes.first!
  }
  @Query(sort: \BibEntryModel.citationKey) private var bibEntries: [BibEntryModel]
  @Environment(\.modelContext) private var modelContext: ModelContext
  public var webView: WKWebView
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
  @AppStorage(AppStorageKeys.includeEmojiSnippets.rawValue) private var includeEmojiSnippets: Bool =
    true
  @AppStorage(AppStorageKeys.storePlainText.rawValue) private var storePlainText: Bool = true

  init(editingNoteId: String?, webView: WKWebView) {
    self.editingNoteId = editingNoteId
    self.webView = webView
    if let _id = editingNoteId {
      let predicate = #Predicate<NoteModel> { $0.id == _id }
      _notes = Query(filter: predicate)
    } else {
      _notes = Query(
        filter: #Predicate<NoteModel> { note in
          false
        })
    }
  }

  var body: some View {
    if editingNoteId == nil {
      NoNoteSelectedView()
    } else {
      WebViewContainerView(
        implementation: WebviewImplementation.mdxEditor,
        editingNoteId: editingNoteId,
        webview: Binding(
          get: {
            webView
          },
          set: { newWebView in
            print("Who the fuck created a new webview?")
          }),
        url: URL.embeddedFlusterUrl(folder: "splitview_mdx_editor_mac", fileName: "index_mac.html"),
        messageHandlerKeys: [
          MdxPreviewWebviewActions.onTagClick.rawValue,
          MdxPreviewWebviewActions.requestNoteData.rawValue,
          SplitviewEditorWebviewActions.onEditorChange.rawValue,
          SplitviewEditorWebviewActions.setWebviewLoaded.rawValue,
          SplitviewEditorWebviewActions.requestSplitviewEditorData.rawValue,
          SplitviewEditorWebviewActions.manualSaveRequest.rawValue
        ],
        messageHandler: messageHandler,
        onLoad: onWebviewLoad,
        mathjaxFontUrl: "/splitview_mdx_editor_mac"
      )
      .confirmationDialog(
        "Delete this note", isPresented: $showDeleteConfirmation, titleVisibility: .visible,
        actions: {
          Button(
            action: {
              if let en = editingNote {
                modelContext.delete(en)
                AppState.shared.setEditingNoteId(editingNoteId: nil)
              }
            },
            label: {
              Label(
                title: {
                  Text("Delete")
                },
                icon: {
                  Image(systemName: "trash")
                })
            })
        }
      )
      .toolbar {
        ToolbarItem(
          placement: .destructiveAction,
          content: {
            Button(
              role: .destructive,
              action: {
                showDeleteConfirmation.toggle()
              },
              label: {
                Label(
                  title: {
                    Text("Delete")
                  },
                  icon: {
                    Image(systemName: "trash")
                  }
                )
              }
            )
          })
        ToolbarItem(
          placement: .primaryAction,
          content: {
            Button(
              action: {
                if let en = editingNote {
                  en.bookmarked = !en.bookmarked
                }
              },
              label: {
                Label(
                  title: {
                    Text("Bookmark")
                  },
                  icon: {
                    if let en = editingNote, en.bookmarked {
                      Image(systemName: "bookmark.fill")
                        .foregroundStyle(.primary)
                    } else {
                      Image(systemName: "bookmark")
                    }
                  })
              }
            )
          })
      }
      .task(
        id: editingNote?.markdown._body, priority: .userInitiated,
        {
          // Parse editing note body every time body is changed.
          // Doing things this way will make the task automatically cancellable
          // when the next change event is fired.
          if let en = editingNote {
            do {
              try await en.preParseIfEdited(modelContext: modelContext)
            } catch {
              print("Error: \(error.localizedDescription)")
            }
          }
        }
      )
      .onAppear {
        if let en = editingNote {
          en.setLastRead()
        }
      }
      .onChange(
        of: editorKeymap,
        {
          Task {
            try? await setCodeEditorKeymap(editorKeymap: editorKeymap)
          }
        }
      )
      .onChange(
        of: lockEditorScrollToPreview,
        {
          Task {
            try? await setLockEditorScrollToPreview(lock: lockEditorScrollToPreview)
          }
        }
      )
      .onChange(
        of: editorSaveMethod,
        {
          Task {
            try? await setEditorSaveMethod(saveMethod: editorSaveMethod)
          }
        }
      )
      .onChange(
        of: editingNote?.id,
        {
          if let en = editingNote {
            en.setLastRead()
          }
        }
      )
      .onChange(
        of: colorScheme,
        {
          Task {
            do {
              try await EditorState.setDarkMode(
                colorScheme: colorScheme, eval: webView.evaluateJavaScript)
            } catch {
              print("Error: \(error.localizedDescription)")
            }
          }
        }
      )
      .frame(maxWidth: .infinity, maxHeight: .infinity)
    }
  }
  func onWebviewLoad() async {
  }
  public func setEditorSaveMethod(saveMethod: EditorSaveMethod) async throws {
    try await EditorState.setEditorSaveMethod(
      saveMethod: saveMethod, eval: webView.evaluateJavaScript)
  }
  public func messageHandler(_ handlerKey: String, _ messageBody: Any) {
    switch handlerKey {
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
      case SplitviewEditorWebviewActions.requestSplitviewEditorData.rawValue:
        Task(priority: .userInitiated) {
          await onWebviewLoad()
        }
      case MdxPreviewWebviewActions.requestNoteData.rawValue:
        Task(priority: .high) {
          await self.onWebviewLoad()
        }
      case SplitviewEditorWebviewActions.requestParsedMdxContent.rawValue:
        Task(priority: .high) {
          await self.onWebviewLoad()
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
        if let en = editingNote {
          let decoder = JSONDecoder()
          let event = try decoder.decode(ManualSaveRequestEvent.self, from: jsonData)
          if event.note_id == en.id {
            en.markdown.body = event.current_note_content
            try await en.preParse(modelContext: modelContext)
            let citations = en.citations.compactMap { cit in
              cit.toEditorCitation(activeCslFile: cslFile)
            }
            try await EditorState.setParsedMdxContent(
              parsedMdxContent: en.markdown.preParsedBody ?? "", citations: citations,
              eval: webView.evaluateJavaScript)
            try await MdxEditorClient.resetErrorState(eval: webView.evaluateJavaScript)
          } else {
            print("Broken state: Found mismatched note id's")
          }
        }
      } catch {
        print("Failed to decode editor update: \(error)")
      }
    }
  }

  public func handleEditorChange(event: EditorChangeEvent) {
    // Don't worry about actually parsing the data. That's all
    // being handled by async tasks managed by SwiftUI for better
    // cancellation management, since it's running onChange.
    if let en = editingNote, en.id == event.note_id {
      let shouldSendModified = event.content != en.markdown.body
      en.markdown.body = event.content
      if shouldSendModified {
        en.setLastRead(setModified: true)
      }
    }
  }

  func setSnippetProps() async throws {
    try await EditorState.setSnippetProps(
      payload: SetSnippetPropsPayload(
        snippetProps: SnippetState(includeEmojiSnippets: includeEmojiSnippets)),
      eval: webView.evaluateJavaScript)
  }
}

#Preview {
  MdxEditorWebview(
    editingNoteId: nil,
    webView: WKWebView(
      frame: .infinite, configuration: getWebViewConfig())
  )
}
