//
//  associate_note_with_bib_entry_item.swift
//  Fluster
//
//  Created by Andrew on 2/21/26.
//

import FlusterBibliography
import FlusterData
import SwiftUI

struct AssociateNoteWithBibEntryItemView: View {
  let item: BibEntryModel
  let editingNote: NoteModel
  @AppStorage(AppStorageKeys.embeddedCslFile.rawValue) private var cslFile: EmbeddedCslFileSwift =
    .apa
  var body: some View {
    let isIncluded = Binding<Bool>(
      get: {
        self.isIncluded()
      },
      set: { newValue in
        if newValue {
          self.addEntry()
        } else {
          self.removeEntry()
        }
      }
    )
    let entryData = item.toBiblatexData()
    HStack(alignment: .center) {
      Toggle(
        isOn: isIncluded,
        label: {}
      )
      .labelsHidden()
      .toggleStyle(.switch)
      VStack(alignment: .leading) {
        Text(item.toFormattedCitation(.fullBibliography, .plaintext, cslFile) ?? item.getTitle())
          .font(.headline)
        if let note = entryData?.getNote() {
          Text(note)
            .font(.footnote)
            .opacity(0.8)
        } else if let abstract = entryData?.getAbstract() {
          Text(abstract)
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
    let itemId = item.id
    do {
      let newCitations = try self.editingNote.citations.filter(
        #Predicate<BibEntryModel> { entry in
          entry.id != itemId
        })
      self.editingNote.citations = newCitations
    } catch {
      print("Error: \(error.localizedDescription)")
    }
  }

  func isIncluded() -> Bool {
    let itemId = item.id
    return self.editingNote.citations.contains(where: { cit in
      return cit.id == itemId
    })
  }

  func addEntry() {
    if !self.isIncluded() {
      self.editingNote.citations.append(self.item)
    }
  }
}
