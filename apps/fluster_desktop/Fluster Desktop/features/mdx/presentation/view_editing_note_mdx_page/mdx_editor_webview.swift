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
  @Environment(\.modelContext) private var modelContext
  @Environment(\.colorScheme) public var colorScheme
  @EnvironmentObject private var appState: AppState
  @Query private var notes: [NoteModel]

  var editingNote: NoteModel? {
    notes.isEmpty ? nil : notes.first!
  }
  @Query(sort: \BibEntryModel.citationKey) private var bibEntries: [BibEntryModel]
  @Binding var webView: WKWebView
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

  init(editingNoteId: String?, webView: Binding<WKWebView>) {
    self.editingNoteId = editingNoteId
    self._webView = webView
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
        webview: $webView,
        url: URL(string: "app://splitview_mdx_editor_mac/index_mac.html")!,
        messageHandlerKeys: [
          MdxPreviewWebviewActions.onTagClick.rawValue,
          MdxPreviewWebviewActions.requestNoteData.rawValue,
          SplitviewEditorWebviewActions.onEditorChange.rawValue,
          SplitviewEditorWebviewActions.setWebviewLoaded.rawValue,
          SplitviewEditorWebviewActions.requestSplitviewEditorData.rawValue
        ],
        messageHandler: messageHandler,
        onLoad: onWebviewLoad
      )
      .toolbar {
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
              try await en.preParse(modelContext: modelContext)
            } catch {
              print("Error: \(error.localizedDescription)")
            }
          }
        }
      )
      .task(
        id: editingNote?.markdown.preParsedBody,
        {
          // When a preParse method succeeds, send the updated parsed content to the editor.
          if let en = editingNote {
            do {
              try await en.preParseIfEdited(modelContext: modelContext)
              let citations: [EditorCitation] = en.citations.compactMap { cit in
                return cit.toEditorCitation(activeCslFile: cslFile)
              }
              try await EditorState.setParsedMdxContent(
                parsedMdxContent: en.markdown.preParsedBody ?? "",
                citations: citations,
                eval: webView.evaluateJavaScript
              )
              // Deprecating this approach entirely in favor of just a handful of buffer based onChange events and the rest
              // as serialized json in less performance critical situations for cross-language compatibility and the ability
              //  to more easily interact with webview state from the lower-level languages.
              //              try await setParsedEditorContentString(note: en)
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
        of: editorThemeDark,
        {
          Task {
            try? await setEditorThemeDark(editorTheme: editorThemeDark)
          }
        }
      )
      .onChange(
        of: editorThemeLight,
        {
          Task {
            try? await setEditorThemeLight(editorTheme: editorThemeLight)
          }
        }
      )
      .onChange(
        of: colorScheme,
        {
          Task {
            try? await setEditorSelectedTheme(
              editorTheme: colorScheme == .dark ? editorThemeDark : editorThemeLight)
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
      .task(
        id: editorSaveMethod,
        {
          try? await setEditorSaveMethod(saveMethod: editorSaveMethod)
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
      .frame(maxWidth: .infinity, maxHeight: .infinity)
    }
  }
  func onWebviewLoad() async {
    if let en = editingNote {
      Task {
        do {
          try await en.preParse(modelContext: modelContext)
         try await EditorState.setInitialEditorState(
            payload: EditorInitialStatePayload(
              note_id: en.id,
              keymap: editorKeymap,
              theme: colorScheme == .dark ? editorThemeDark : editorThemeLight,
              allCitationIds: bibEntries.compactMap(\.citationKey),
              value: en.markdown.body,
              parsedValue: en.markdown.preParsedBody ?? "",
              haveSetInitialValue: true,
              snippetProps: SnippetState(includeEmojiSnippets: true),
              lockEditorScrollToPreview: lockEditorScrollToPreview,
              saveMethod: editorSaveMethod),
            eval: webView.evaluateJavaScript
          )
        } catch {
          print("Error initializing Mdx Editor Webview: \(error.localizedDescription)")
        }
      }
    }
  }
  public func setEditorSaveMethod(saveMethod: EditorSaveMethod) async throws {
    try await EditorState.setEditorSaveMethod(
      saveMethod: saveMethod, eval: webView.evaluateJavaScript)
  }
  public func messageHandler(_ handlerKey: String, _ messageBody: Any) {
    switch handlerKey {
      case SplitviewEditorWebviewActions.onEditorChange.rawValue:
        handleEditorChange(newValue: messageBody as! String)
      case SplitviewEditorWebviewActions.requestSplitviewEditorData.rawValue:
        Task(priority: .userInitiated) {
          await onWebviewLoad()
        }
      case MdxPreviewWebviewActions.requestNoteData.rawValue:
        if let en = editingNote {
          Task(priority: .high) {
            do {
              try await self.setParsedEditorContentString(note: en)
            } catch {
              print("Error: \(error.localizedDescription)")
            }
          }
        }
      case SplitviewEditorWebviewActions.requestParsedMdxContent.rawValue:
        Task(priority: .high) {
          if let en = editingNote {
            do {
              try await setParsedEditorContentString(
                note: en
              )
            } catch {
              print("Error: \(error.localizedDescription)")
            }
          }
        }
      default:
        return
    }
  }
  public func handleEditorChange(newValue: String) {
    // Don't worry about actually parsing the data. That's all
    // being handled by async tasks managed by SwiftUI for better
    // cancellation management, since it's running onChange.
    if let en = editingNote {
      en.markdown.body = newValue
      en.setLastRead(setModified: true)
    }
  }

  func setSnippetProps() async throws {
    var builder = FlatBufferBuilder(initialSize: 1024)
    let ctiationIdsVectorOffset = builder.createVector(
      ofStrings: bibEntries.compactMap(\.citationKey))
    let data = Snippets_GetSnippetPropsBuffer.createGetSnippetPropsBuffer(
      &builder, citationIdsVectorOffset: ctiationIdsVectorOffset)
    builder.finish(offset: data)
    try await webView.evaluateJavaScript(
      """
      window.setSnippetProps(\(builder.sizedByteArray))
      """)
  }
}

#Preview {
  MdxEditorWebview(
    editingNoteId: nil,
    webView: .constant(
      WKWebView(
        frame: .infinite, configuration: getWebViewConfig())
    ))
}
