//
//  bibliography_search_results_view.swift
//  Fluster
//
//  Created by Andrew on 11/5/25.
//

import FlusterSwift
import SwiftData
import SwiftUI

struct BibliographySearchResultsViewQueryWrapped: View {
  @StateObject private var bibtexEditorContainer =
    BibtexEditorWebviewContainer(bounce: true, scrollEnabled: true)
  @Query(sort: \BibEntryModel.ctime) var bibEntries: [BibEntryModel]
  @Environment(\.modelContext) var modelContext
  @State private var confirmationModalOpen: Bool = false
  @State private var deletingBibEntry: BibEntryModel? = nil
  @Binding var searchQuery: String
  @Binding var editingNote: NoteModel?

  init(searchQuery: Binding<String>, editingNote: Binding<NoteModel?>) {
    _searchQuery = searchQuery
    _editingNote = editingNote

    if !searchQuery.wrappedValue.isEmpty {
      let query_cantUseGetterInPredicate = searchQuery.wrappedValue
      _bibEntries = Query(
        filter: #Predicate<BibEntryModel> { entry in
          entry.data.localizedStandardContains(
            query_cantUseGetterInPredicate
          )
        },
        sort: [SortDescriptor(\BibEntryModel.utime, order: .reverse)],
        animation: .default
      )
    }
  }

  var filteredEntries: [BibEntryModel] {
    if searchQuery.trimmingCharacters(in: .whitespacesAndNewlines).isEmpty {
      return bibEntries
    }
    let searchQueryLowercased = searchQuery.lowercased()
    return bibEntries.filter({
      $0.data.lowercased().contains(searchQueryLowercased)
    })
  }

  var body: some View {
    if bibEntries.isEmpty {
      EmptyBibSearchResultsView(
        bibtexEditorContainer: bibtexEditorContainer
      )
    } else {
      List {
        ForEach(bibEntries, id: \.id) { item in
          NavigationLink(
            destination: {
              ByBibEntrySearchResults(
                bibEntryId: item.id,
                editingNote: $editingNote
              )
            },
            label: {
              BibEntrySearchResultItemView(
                item: item,
              )
            }
          )
          .swipeActions(
            edge: .leading,
            content: {
              Button(
                "Delete",
                role: .destructive
              ) {
                deletingBibEntry = item
                confirmationModalOpen = true
              }
            }
          )
        }
      }
      .onChange(
        of: confirmationModalOpen,
        {
          if !confirmationModalOpen {
            deletingBibEntry = nil
          }
        }
      )
      .confirmationDialog(
        "Delete this bibliography entry?",
        isPresented: $confirmationModalOpen,
      ) {
        Button("Delete") {
          if let deletingBibEntryExists = deletingBibEntry {
            modelContext.delete(deletingBibEntryExists)
            deletingBibEntry = nil
          }
        }
      } message: {
        Text("Delete this bibliography entry?")
      }
      .searchable(
        text: $searchQuery,
        placement: .toolbarPrincipal,
        prompt: "Search"
      )
    }
  }
}

struct BibliographySearchResultsView: View {
  @State private var searchQuery: String = ""
  let bibtexEditorContainer = BibtexEditorWebviewContainer()
  @Binding var editingNote: NoteModel?
  var body: some View {
    BibliographySearchResultsViewQueryWrapped(
      searchQuery: $searchQuery,
      editingNote: $editingNote
    )
    .toolbar(content: {
      NavigationLink(
        destination: CreateBibEntrySheetView(
          editingBibEntry: .constant(nil),
          ignoreEditingNote: true,
          container: bibtexEditorContainer
        ),
        label: {
          Label("Create", systemImage: "plus")
        }
      )
    })
  }
}

#Preview {
  BibliographySearchResultsView(
    editingNote: .constant(nil)
  )
}
