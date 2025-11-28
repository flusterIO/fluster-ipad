//
//  bib_list_item_view.swift
//  Fluster
//
//  Created by Andrew on 11/3/25.
//

import SwiftData
import SwiftUI
import SwiftyBibtex
import FlusterSwift


struct BibEntryListItemView: View {
    var item: BibEntryModel
    @Binding var editing: BibEntryModel?
    let container: BibtexEditorWebviewContainer
    @State private var confirmationModalModel: Bool = false
    @Environment(\.modelContext) var modelContext

    var body: some View {
        Text(item.getTitle())
            .font(.subheadline)
            .swipeActions(content: {
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
            })
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
