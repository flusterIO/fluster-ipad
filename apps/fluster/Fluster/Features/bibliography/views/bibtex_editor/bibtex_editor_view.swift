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
  @State private var newItemData: String = ""
  @State private var showCantSaveEmpty: Bool = false
  @Environment(\.modelContext) private var modelContext: ModelContext
  @Environment(\.colorScheme) private var colorScheme: ColorScheme
  @Environment(\.dismiss) private var dismiss

  init(
    editingItem: Binding<BibEntryModel?>, editingNote: Binding<NoteModel?>,
    associateWithEditingNote: Bool = true, webView: Binding<WKWebView>
  ) {
    self._editingItem = editingItem
    self._editingNote = editingNote
    self._webView = webView
    self.associateWithEditingNote = associateWithEditingNote
  }

  public var body: some View {
    IosWebviewContainer(
      implementation: .bibEditor, editingNote: $editingNote, webView: $webView,
      show: $showWebView,
      url: URL.embeddedFlusterUrl(
        folder: "bibtex_editor_webview_ipad", fileName: "index_ipad.html"), messageHandlerKeys: [],
      messageHandler: self.messageHandler, onLoad: nil
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

  func messageHandler(_ messageKey: String, _ messageBody: Any) {
  }
}
