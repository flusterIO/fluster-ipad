//
//  create_bib_entry_view.swift
//  Fluster
//
//  Created by Andrew on 2/18/26.
//

import FlusterBibliography
import FlusterData
import FlusterSwift
import FlusterWebviewClients
import SwiftData
import SwiftUI
import WebKit

struct BibtexEditorWebview: View {
  let editingNoteId: String?
  /// If editingItem != nil, the editingItem will be updated. Else, a new bibEntry will be created
  @Binding public var editingItem: BibEntryModel?
  @State private var newItemData: String = ""
  @State private var showCantSaveEmpty: Bool = false
  @State private var webView: WKWebView = WKWebView(
    frame: .infinite, configuration: getWebViewConfig()
  )
  @Environment(\.modelContext) private var modelContext: ModelContext
  @Environment(\.colorScheme) private var colorScheme: ColorScheme
  @Environment(\.dismiss) private var dismiss
  @AppStorage(AppStorageKeys.editorKeymap.rawValue) private var editorKeymap: CodeEditorKeymap =
    .base
  @AppStorage(AppStorageKeys.editorThemeDark.rawValue) private var editorThemeDark:
    CodeEditorTheme = .dracula
  @AppStorage(AppStorageKeys.editorThemeLight.rawValue) private var editorThemeLight:
    CodeEditorTheme = .materialLight
  /// If associateWithEditingNote == true, *new* BibEntryModel will be associated with the note currently being edited.
  let associateWithEditingNote: Bool = true
  @Query private var notes: [NoteModel]
  var editingNote: NoteModel? {
    notes.isEmpty ? nil : notes.first
  }

  init(editingNoteId: String?, editingBibEntry: Binding<BibEntryModel?>) {
    self.editingNoteId = editingNoteId
    self._editingItem = editingBibEntry
    if let id = editingNoteId {
      self._notes = Query(
        filter: #Predicate<NoteModel> { note in
          note.id == id
        }
      )
    } else {
      self._notes = Query(
        filter: #Predicate<NoteModel> { note in
          false
        }
      )
    }
  }

  var body: some View {
    WebViewContainerView(
      implementation: WebviewImplementation.bibEditor,
      editingNoteId: editingNoteId,
      webview: $webView,
      url: URL.embeddedFlusterUrl(folder: "bibtex_editor_webview_mac", fileName: "index_mac.html"),
      messageHandlerKeys: [
        SplitviewEditorWebviewActions.onBibEditorChange.rawValue,
        BibtexEditorWebviewActions.requestBibtexEditorData.rawValue,
        BibtexEditorWebviewActions.setWebviewLoaded.rawValue
      ],
      messageHandler: messageHandler,
      onLoad: onWebviewLoad
    )
    .sheet(
      isPresented: $showCantSaveEmpty,
      content: {
        NotificationConfirmationContainerView(
          buttonText: "Go back",
          content: {
            Text("You can't save an empty file. Please enter valid biblatex.")
          })
      }
    )
    .toolbar(content: {
      ToolbarItem(
        id: "save-bib-entry", placement: .primaryAction,
        content: {
          Button(
            action: {
              handleSave()
            },
            label: {
              Label(
                title: {
                  Text("Success")
                },
                icon: {
                  Image(systemName: "checkmark")
                    .foregroundStyle(Color.accentColor)
                })
            }
          )
        })
    })
    .onChange(
      of: editorKeymap,
      {
        Task {
          try? await setEditorKeymap(editorKeymap: editorKeymap)
        }
      }
    )
  }
  func handleSave() {
    // Nothing to do if editingItem != nil, item should have been updated as the content was updated. Thanks to Apple's magic, that's all that needs to be done, which still blows my mind.
    if editingItem == nil {
      if newItemData.trimmingCharacters(in: .whitespacesAndNewlines).isEmpty {
        showCantSaveEmpty = true
      } else {
        let splitByEntry = FlusterBibliography.splitBiblatexToRawStrings(fileContent: newItemData)
        for entryText in splitByEntry {
          let item = BibEntryModel(data: entryText, notes: editingNote == nil ? [] : [editingNote!])
          if let en = editingNote, associateWithEditingNote {
            en.citations.append(item)
          } else {
            modelContext.insert(item)
          }
          modelContext.insert(item)
        }

        Task(priority: .high) {
          do {
            try await EditorState.setEditorBibContent(content: "", eval: webView.evaluateJavaScript)
          } catch {
            print("Error: \(error.localizedDescription)")
          }
        }
        dismiss()
      }
    } else {
      // TODO: Need to handle updating of bibentry here.
    }
  }
  func setEditorKeymap(editorKeymap: CodeEditorKeymap) async throws {
    try await EditorState.setEditorKeymap(keymap: editorKeymap, eval: webView.evaluateJavaScript)
  }
  func handleEditorChange(newValue: String) {
    if let ei = editingItem {
      ei.data = newValue
    } else {
      newItemData = newValue
    }
  }
  func setEntryBodyByString(entryBody: String) {
    Task {
      do {
        try await BibtexEditorClient.setBibEntryContent(
          entryBody: entryBody, evalutateJavaScript: webView.evaluateJavaScript)
      } catch {
        print("Error setting bibliography editor data: \(error.localizedDescription)")
      }
    }
  }
  func setBibEntryData(data: BibEntryModel) {
    Task {
      do {
        try await BibtexEditorClient.setBibEntryContent(
          entryBody: data.data, evalutateJavaScript: webView.evaluateJavaScript)
      } catch {
        print("Error setting bibliography editor data: \(error.localizedDescription)")
      }
    }
  }
  func messageHandler(_ msgKey: String, msgBody: Any) {
    switch msgKey {
      case SplitviewEditorWebviewActions.onBibEditorChange.rawValue:
        handleEditorChange(newValue: msgBody as! String)
      case BibtexEditorWebviewActions.requestBibtexEditorData.rawValue:
        Task(priority: .high) {
          await onWebviewLoad()
        }
      case BibtexEditorWebviewActions.setWebviewLoaded.rawValue:
        print("Webview Loaded...")
      default:
        return
    }
  }
  func onWebviewLoad() async {
    if let entry = editingItem {
      setBibEntryData(data: entry)
    } else {
      setEntryBodyByString(entryBody: newItemData)
    }
    Task {
      try? await setEditorKeymap(editorKeymap: editorKeymap)
    }
  }
}
