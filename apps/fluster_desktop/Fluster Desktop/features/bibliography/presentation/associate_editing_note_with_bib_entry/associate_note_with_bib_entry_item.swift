//
//  associate_note_with_bib_entry_item.swift
//  Fluster
//
//  Created by Andrew on 2/21/26.
//

import FlusterBibliography
import FlusterData
import SwiftData
import SwiftUI

struct BibSummaryData {
  let title: String
  let summary: String?
}

struct AssociateNoteWithBibEntryItemView: View {
  let item: BibEntryModel
  var itemId: String {
    item.id
  }
  let editingNoteId: String
  @AppStorage(AppStorageKeys.embeddedCslFile.rawValue) private var cslFile: EmbeddedCslFileSwift =
    .apa
  @State private var data: BibSummaryData? = nil
  @Environment(\.modelContext) private var modelContext: ModelContext
  @Query private var notes: [NoteModel]
  var editingNote: NoteModel? {
    notes.isEmpty ? nil : notes.first
  }
  var formatted: FormattedCitation? {
    item.safelyGetFormatted(activeCslFormat: cslFile)
  }

  init(item: BibEntryModel, editingNoteId: String) {
    self.item = item
    self.editingNoteId = editingNoteId
    var descriptor = FetchDescriptor<NoteModel>(
      predicate: #Predicate<NoteModel> { note in
        note.id == editingNoteId
      }
    )
    descriptor.fetchLimit = 1
    self._notes = Query(
      descriptor
    )
  }

  var body: some View {
    let isIncluded = Binding<Bool>(
      get: {
        self.isIncluded()
      },
      set: { newValue in
        if newValue {
          Task(priority: .userInitiated) {
            self.addEntry()
          }
        } else {
          self.removeEntry()
        }
      }
    )
    HStack(alignment: .center) {
      Toggle(
        isOn: isIncluded,
        label: {}
      )
      .labelsHidden()
      .toggleStyle(.switch)
      VStack(alignment: .leading) {
        Text(formatted?.title ?? "No title found")
          .font(.headline)
          .frame(maxWidth: .infinity, alignment: .leading)
        if let note = formatted?.note {
          Text(note)
            .frame(maxWidth: .infinity, alignment: .leading)
            .lineLimit(5)
            .font(.footnote)
            .opacity(0.8)
        } else if let abstract = formatted?.abstract {
          Text(abstract)
            .frame(maxWidth: .infinity, alignment: .leading)
            .lineLimit(5)
            .font(.footnote)
            .opacity(0.8)
        }
      }
      .frame(maxWidth: .infinity)
      Spacer()
    }
    .padding()
    .glassEffect(in: .rect(cornerRadius: 16))
  }

  func removeEntry() {
    if let en = self.editingNote {
      en.removeCitation(citation: self.item)
      en.setLastRead(setModified: true)
    }
  }

  func isIncluded() -> Bool {
    let itemId = item.id
    return self.editingNote?.citations.contains(where: { cit in
      return cit.id == itemId
    }) ?? false
  }

  func addEntry() {
    if let en = self.editingNote {
      if !self.isIncluded() {
        en.addCitation(citation: self.item, strategy: .userAdded)
        en.setLastRead(setModified: true)
        do {
          print("Saving after appending...")
          try modelContext.save()
        } catch {
          print("Error: \(error.localizedDescription)")
        }
      } else {
        print("Attempted to add a citation that is already included.")
      }
    }
  }
}
