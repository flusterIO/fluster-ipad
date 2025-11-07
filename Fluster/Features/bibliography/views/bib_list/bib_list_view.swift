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
    @State private var editingInputValue: String = ""
    var body: some View {
        HStack{
            List(items, id: \.id) {item in
                BibEntryListItemView(item: item, editing: $editing)
            }
        }
        .sheet(isPresented: $editingSheetOpen, content: {
            CreateBibEntrySheetView(
                inputValue: $editingInputValue,
                isPresented: $editingSheetOpen,
                editing: $editing
            )
        })
        .onChange(of: editing, {
            if editing != nil {
                editingSheetOpen = true
                editingInputValue = editing!.data
            } else {
                editingSheetOpen = false
            }
        })
    }
}

#Preview {
    BibListView(items: [], editing: .constant(nil))
}
