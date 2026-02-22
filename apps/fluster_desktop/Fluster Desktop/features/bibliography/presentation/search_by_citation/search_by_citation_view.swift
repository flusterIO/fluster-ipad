//
//  search_by_citation_view.swift
//  Fluster
//
//  Created by Andrew on 2/21/26.
//

import SwiftUI
import FlusterData
import SwiftData

struct SearchByCitationView: View {
    let citation: BibEntryModel
    @Query var notes: [NoteModel]
    init(citation: BibEntryModel) {
        self.citation = citation
        let citationId = citation.id
        self._notes = Query(
            FetchDescriptor(
                predicate: #Predicate<NoteModel> { note in
                    note.citations.contains(where: { entry in
                        entry.id == citationId
                    })
                }
            )
        )
    }
    var body: some View {
        NoteSearchResultsListView(notes: notes)
    }
}
