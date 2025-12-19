//
//  bib_list_view.swift
//  Fluster
//
//  Created by Andrew on 11/3/25.
//

import FlusterSwift
import SwiftData
import SwiftUI

struct BibListView: View {
    @Environment(ThemeManager.self) private var themeManager: ThemeManager
    @Environment(\.modelContext) var modelContext
    var items: [BibEntryModel]
    @Binding var editingBibEntry: BibEntryModel?
    @Binding var editingNote: NoteModel?
    let editorContainer: BibtexEditorWebviewContainer

    var body: some View {
        ZStack(alignment: .bottomTrailing) {
            if items.isEmpty {
                Text("No bibliography entries found.")
                    .font(.title)
            } else {
                List {
                    ForEach(items, id: \.id) { item in
                        NavigationLink(
                            destination: ByBibEntrySearchResults(
                                bibEntryId: item.id,
                                editingNote: $editingNote
                            ),
                            label: {
                                BibEntrySearchResultItemView(item: item)
                            }
                        )
                        .swipeActions(
                            edge: .leading,
                            allowsFullSwipe: true,
                            content: {
                                Button(
                                    action: {
                                        editingNote?.diassociateBibEntry(
                                            bibEntry: item
                                        )
                                    },
                                    label: {
                                        Label(
                                            "Disassociate",
                                            systemImage: "x.circle.fill"
                                        )
                                    }
                                )
                            }
                        )
                    }
                }
            }
        }
    }

}

#Preview {
    BibListView(
        items: [],
        editingBibEntry: .constant(nil),
        editingNote: .constant(nil),
        editorContainer: BibtexEditorWebviewContainer(
            bounce: true,
            scrollEnabled: true,
        )
    )
}
