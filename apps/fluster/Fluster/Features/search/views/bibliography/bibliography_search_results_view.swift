//
//  bibliography_search_results_view.swift
//  Fluster
//
//  Created by Andrew on 11/5/25.
//

import SwiftData
import SwiftUI

struct BibliographySearchResultsViewQueryWrapped: View {
    @StateObject private var bibtexEditorContainer =
        BibtexEditorWebviewContainer()
    @Query(sort: \BibEntryModel.ctime) var bibEntries: [BibEntryModel]
    @Binding var editingNote: NoteModel?
    @Binding var searchQuery: String

    init(editingNote: Binding<NoteModel?>, searchQuery: Binding<String>) {
        _editingNote = editingNote
        _searchQuery = searchQuery

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
                    BibEntrySearchResultItemView(
                        item: item,
                        editingNote: $editingNote,
                    )
                }
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
    @Binding var editingNote: NoteModel?
    let bibtexEditorContainer = BibtexEditorWebviewContainer()
    var body: some View {
        BibliographySearchResultsViewQueryWrapped(
            editingNote: $editingNote,
            searchQuery: $searchQuery
        )
        .toolbar(content: {
            ToolbarItemGroup(
                placement: .topBarTrailing,
                content: {
                    NavigationLink(
                        destination: CreateBibEntrySheetView(
                            editingBibEntry: .constant(nil),
                            container: bibtexEditorContainer
                        ),
                        label: {
                            Label("Create", systemImage: "plus")
                        }
                    )
                }
            )
        })
    }
}

#Preview {
    BibliographySearchResultsView(
        editingNote: .constant(nil),
    )
}
