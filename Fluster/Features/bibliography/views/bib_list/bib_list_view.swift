//
//  bib_list_view.swift
//  Fluster
//
//  Created by Andrew on 11/3/25.
//

import SwiftUI
import SwiftData

struct BibListView: View {
    var items: [BibEntryModel]
    @Binding var editing: BibEntryModel?
    @State private var editingSheetOpen: Bool = false
    var body: some View {
        HStack{
            List(items, id: \.id) {item in
                BibEntryListItemView(item: item, editing: $editing)
            }
        }
        .sheet(isPresented: $editingSheetOpen, content: {
            CreateBibEntrySheetView(isPresented: $editingSheetOpen)
        })
        .onChange(of: editing, {
            if editing != nil {
                editingSheetOpen = true
            }
        })
    }
}

#Preview {
    BibListView(items: [], editing: .constant(nil))
}
