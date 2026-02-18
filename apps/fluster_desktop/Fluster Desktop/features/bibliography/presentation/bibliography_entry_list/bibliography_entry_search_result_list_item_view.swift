//
//  bibliography_entry_search_result_list_item_view.swift
//  Fluster
//
//  Created by Andrew on 2/18/26.
//

import SwiftUI
import FlusterData


struct BibliographyEntrySearchResultListItemView: View {
    let item: BibEntryModel
    var body: some View {
        VStack {
            Text(item.title)
                .font(.title2)
                .padding()
        }
        .glassEffect(in: .rect(cornerRadius: 16))
        .frame(maxWidth: 1080, maxHeight: .infinity)
        .padding()
    }
}

//#Preview {
//    BibliographyEntrySearchResultListItemView(item: BibEntryModel(
//        data: <#T##String#>
//    ))
//}
