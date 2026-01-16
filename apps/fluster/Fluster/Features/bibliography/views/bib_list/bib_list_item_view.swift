//
//  bib_list_item_view.swift
//  Fluster
//
//  Created by Andrew on 11/3/25.
//

import FlusterSwift
import SwiftData
import SwiftUI
import SwiftyBibtex
import FlusterData

struct BibEntryListItemView: View {
  var item: BibEntryModel
  @Binding var editing: BibEntryModel?
  let container: BibtexEditorWebviewContainer
  @State private var confirmationModalModel: Bool = false
  @Environment(\.modelContext) var modelContext
  @Environment(\.dismiss) var dismiss
  @Environment(NoteModel.self) var editingNote: NoteModel?

  var body: some View {
    Text(item.getTitle())
      .font(.subheadline)
      .swipeActions(
        edge: .leading,
        content: {
          Button(
            action: {
              confirmationModalModel = true
            },
            label: {
              Label("Delete", systemImage: "trash")
            }
          )
          Button(
            action: {
              editing = item
            },
            label: {
              Label("Edit", systemImage: "slider.horizontal.3")
            }
          )
          Button("Disassociate") {
            if let editingNoteExists = editingNote {
              editingNoteExists.citations.removeAll {
                $0.id == item.id
              }
            }
          }
        }
      )
      .confirmationDialog(
        "Delete this bibliography entry?",
        isPresented: $confirmationModalModel,
      ) {
        Button("Cancel", role: .cancel) {}
        Button("Delete") {
          modelContext.delete(item)
        }
      } message: {
        Text("Delete this bibliography entry?")
      }
  }
}
