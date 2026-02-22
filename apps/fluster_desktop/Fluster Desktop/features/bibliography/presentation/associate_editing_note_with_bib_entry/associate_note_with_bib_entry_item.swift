//
//  associate_note_with_bib_entry_item.swift
//  Fluster
//
//  Created by Andrew on 2/21/26.
//

import FlusterBibliography
import FlusterData
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
  let editingNote: NoteModel
  @AppStorage(AppStorageKeys.embeddedCslFile.rawValue) private var cslFile: EmbeddedCslFileSwift =
    .apa
  @State private var data: BibSummaryData? = nil
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
        Text(data?.title ?? "No title found")
          .font(.headline)
          .frame(maxWidth: .infinity, alignment: .leading)
        if let note = data?.summary {
          Text(note)
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
    .task {
      if self.data == nil {
        let entryData = parseBiblatexString(biblatexContent: item.data).first
        let cslContent = cslFile.toFlusterBibliographyCslFile()
        Task.detached {
          if let res = entryData {
            let s = res.formatBibliographyCitation(
              cslContent: cslContent,
              cslLocale: getCslLocaleFileContent(), renderMethod: .plaintext)
            let _data = BibSummaryData(
              title: s ?? res.getTitle() ?? "No title found",
              summary: res.getNote() ?? res.getAbstract())
            await MainActor.run {
              self.data = _data
            }
          }
        }
      }
    }
  }

  func removeEntry() {
    let itemId = item.id
    do {
      let newCitations = try self.editingNote.citations.filter(
        #Predicate<BibEntryModel> { entry in
          entry.id != itemId
        })
      self.editingNote.citations = newCitations
      self.editingNote.setLastRead(setModified: true)
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
      self.editingNote.setLastRead(setModified: true)
    }
  }
}
