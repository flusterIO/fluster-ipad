//
//  bib_list_item_view.swift
//  Fluster
//
//  Created by Andrew on 11/3/25.
//

import SwiftUI
import SwiftyBibtex

struct BibEntryListItemView: View {
    var item: BibEntryModel
    @Binding var editing: BibEntryModel?
    var body: some View {
        Text(item.parse()?.publications[0].fields["title"] ?? "--")
            .font(.subheadline)
            .swipeActions(content: {
                Button(action: {
                   print("Editing here...")
                    editing = item
                }, label: {
                    Label("Edit", systemImage: "slider.horizontal.3")
                })
            })
    }
}

