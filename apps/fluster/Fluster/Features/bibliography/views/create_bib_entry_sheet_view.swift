//
//  create_bib_entry_sheet_view.swift
//  Fluster
//
//  Created by Andrew on 11/3/25.
//

import FlusterBibliography
import FlusterData
import FlusterSwift
import FlusterWebviewClients
import SwiftData
import SwiftUI
import WebKit

struct CreateBibEntrySheetView: View {
  @State private var newEntryValue: String = ""

  @State private var webView: WKWebView = WKWebView(
    frame: .infinite, configuration: getConfig()
  )
  @Environment(\.modelContext) var modelContext
  @Environment(\.dismiss) var dismiss
  @Environment(ThemeManager.self) private var themeManager: ThemeManager
  @Binding private var editingNote: NoteModel?
  @Binding var editingBibEntry: BibEntryModel?

  let ignoreEditingNote: Bool

  init(
    ignoreEditingNote: Bool,
    editingNote: Binding<NoteModel?>,
    editingBibEntry: Binding<BibEntryModel?>,
  ) {
    self.ignoreEditingNote = ignoreEditingNote
    self._editingNote = editingNote
    self._editingBibEntry = editingBibEntry
  }

  var body: some View {
    VStack {
      Spacer(minLength: 8)
      HStack(alignment: .center) {
        Text("Enter valid bibtex entry.")
          .font(.caption)
      }
      BibtexEditorWebview(
        editingItem: $editingBibEntry, editingNote: $editingNote,
        associateWithEditingNote: !ignoreEditingNote, webView: $webView)
      Spacer(minLength: 8)
      HStack(alignment: .center) {
        Spacer()
        Button("Cancel") {
          newEntryValue = ""
          dismiss()
        }
        Spacer()
        Button(editingBibEntry == nil ? "Create" : "Update") {
          let inputValue =
            editingBibEntry == nil
            ? self.newEntryValue : editingBibEntry!.data
          if inputValue.trimmingCharacters(
            in: .whitespacesAndNewlines
          ).isEmpty {
            return
          }
          if editingBibEntry == nil {
            let splitContent = splitBiblatexToRawStrings(fileContent: inputValue)
            for bibEntryText in splitContent {
              // -- If the model should be created new. --
              let newEntry = BibEntryModel(
                data: bibEntryText,
                notes: ignoreEditingNote || editingNote == nil ? [] : [editingNote!])
              if let _editingNote = editingNote, !ignoreEditingNote {
                _editingNote.addCitation(citation: newEntry, strategy: .userAdded)
              } else {
                modelContext.insert(newEntry)
              }
            }
            newEntryValue = ""
            setEditorContent(content: "")
            dismiss()
          } else {
            // -- If the model needs to be updated. --
            editingBibEntry!.data = inputValue
            setEditorContent(content: "")
            dismiss()
          }
        }
        .buttonStyle(.glassProminent)
        Spacer()
      }
      .padding()
      Spacer(minLength: 8)
    }
  }
  func setEditorContent(content: String) {
    Task {
      do {
        try await EditorState.setEditorBibContent(
          content: content, eval: webView.evaluateJavaScript)
      } catch {
        print("Error: \(error.localizedDescription)")
      }
    }
  }
}
