//
//  bib_list_view.swift
//  Fluster
//
//  Created by Andrew on 11/3/25.
//

import SwiftData
import SwiftUI

struct BibListView: View {
    var items: [BibEntryModel]
    @Binding var editing: BibEntryModel?
    @State private var editingSheetOpen: Bool = false
    @State private var editingInputValue: String = ""
    @Environment(ThemeManager.self) private var themeManager: ThemeManager

    var body: some View {
        ZStack(alignment: .bottomTrailing) {
            HStack {
                List(items, id: \.id) { item in
                    BibEntryListItemView(item: item, editing: $editing)
                }
            }
            Button {
                editingInputValue = ""
                editing = nil
                editingSheetOpen = true
            } label: {
                Image(systemName: "plus")
                    .imageScale(.large)
                    .foregroundStyle(themeManager.theme.primary_foreground)
                    .clipShape(Circle())
                    .padding()
            }
            .buttonStyle(.borderedProminent)
            .padding()
        }
        .fullScreenCover(
            isPresented: $editingSheetOpen,
            content: {
                CreateBibEntrySheetView(
                    inputValue: $editingInputValue,
                    isPresented: $editingSheetOpen,
                    editing: $editing
                )
            }
        )
        .onChange(
            of: editing,
            {
                if editing != nil {
                    editingSheetOpen = true
                    editingInputValue = editing!.data
                } else {
                    editingInputValue = ""
                }
            }
        )
    }
}

#Preview {
    BibListView(items: [], editing: .constant(nil))
}
