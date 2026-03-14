//
//  bibtex_editor_view.swift
//  Fluster
//
//  Created by Andrew on 3/13/26.
//

import FlusterBibliography
import FlusterData
import FlusterSwift
import FlusterWebviewClients
import SwiftData
import SwiftUI
import WebKit

public struct BibtexEditorWebview: View {
  @Binding public var editingItem: BibEntryModel?
  @Binding public var editingNote: NoteModel?
  @State private var showWebView: Bool = false
  public var associateWithEditingNote: Bool = true
  @Binding private var webView: WKWebView
  @Binding private var newItemData: String
  @Binding private var errorMessage: EditorErrorMessage?
  @Environment(\.modelContext) private var modelContext: ModelContext
  @Environment(\.colorScheme) private var colorScheme: ColorScheme
  @Environment(\.dismiss) private var dismiss

  init(
    editingItem: Binding<BibEntryModel?>,
    editingNote: Binding<NoteModel?>,
    associateWithEditingNote: Bool = true,
    webView: Binding<WKWebView>,
    errorMessage: Binding<EditorErrorMessage?>,
    newItemContent: Binding<String>
  ) {
    self._newItemData = newItemContent
    self._errorMessage = errorMessage
    self._editingItem = editingItem
    self._editingNote = editingNote
    self._webView = webView
    self.associateWithEditingNote = associateWithEditingNote
  }

  public var body: some View {
    let errorModalOpen = Binding(
      get: {
        return errorMessage != nil
      },
      set: { newValue in
        if newValue == false {
          errorMessage = nil
        }
      })
    IosWebviewContainer(
      implementation: .bibEditor,
      editingNote: $editingNote,
      webView: $webView,
      show: $showWebView,
      url: URL.embeddedFlusterUrl(
        folder: "bibtex_editor_webview_ipad",
        fileName: "index_ipad.html"),
      messageHandlerKeys: [
        SplitviewEditorWebviewActions.onBibEditorChange.rawValue
      ],
      messageHandler: self.messageHandler,
      onLoad: nil
    )
    .alert(
      "Editor Error", isPresented: errorModalOpen,
      actions: {
        Button(
          action: {
            errorMessage = nil
          },
          label: {
            Label(
              title: {
                Text(
                  ["Got it", "Understood", "Confirm", "OK", "Continue"].randomElement() ?? "Got it")
              },
              icon: {
                Image(systemName: "shield.lefthalf.filled.badge.checkmark")
              })
          })
      },
      message: {
        Text(errorMessage?.title ?? "Unknown Error")
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
  }

  func handleSave() {
    if newItemData.trimmingCharacters(in: .whitespacesAndNewlines).isEmpty {
      errorMessage = EditorErrorMessage(
        id: .attemptToSaveEmptyBibEntry, title: "Can't save an empty bibliography entry.",
        desc: nil)
      return
    }
    // Nothing to do if editingItem != nil, item should have been updated as the content was updated. Thanks to Apple's magic, that's all that needs to be done, which still blows my mind.
    if let editingItem = self.editingItem {
      let splitByEntry = FlusterBibliography.splitBiblatexToRawStrings(fileContent: newItemData)
      if splitByEntry.count != 1 {
        // TODO: Send a notification to the user here.
        errorMessage = EditorErrorMessage(
          id: .attemptToUpdateOneBibEntryAsMany,
          title: "Cannot update a single bib entry to contain multiple entries.", desc: nil)
        return
      } else {
        editingItem.data = splitByEntry[0]
        newItemData = ""
      }
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
  }

  func handleBibEditorChange(_ bibContent: String) {
    newItemData = bibContent
  }

  func messageHandler(_ messageKey: String, _ messageBody: Any) {
    print("Message Received: \(messageKey)")
    switch messageKey {
      case SplitviewEditorWebviewActions.onBibEditorChange.rawValue:
        self.handleBibEditorChange(messageBody as! String)
      default:
        return
    }
  }
}
