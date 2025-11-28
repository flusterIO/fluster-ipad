//
//  bib_list_view.swift
//  Fluster
//
//  Created by Andrew on 11/3/25.
//

import SwiftData
import SwiftUI
import FlusterSwift

struct BibListView: View {
    var items: [BibEntryModel]
    @Binding var editing: BibEntryModel?
    let editorContainer: BibtexEditorWebviewContainer
    @Environment(ThemeManager.self) private var themeManager: ThemeManager
    @Environment(\.modelContext) var modelContext

    var body: some View {
        ZStack(alignment: .bottomTrailing) {
            List {
                ForEach(items, id: \.id) { item in
                    BibEntryListItemView(
                        item: item,
                        editing: $editing,
                        container: editorContainer
                    )
                }
            }
        }
    }

}

#Preview {
    BibListView(
        items: [],
        editing: .constant(nil),
        editorContainer: BibtexEditorWebviewContainer()
    )
}
