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
    var items: [BibEntryModel]
    @Binding var editing: BibEntryModel?
    let editorContainer: BibtexEditorWebviewContainer
    @Environment(ThemeManager.self) private var themeManager: ThemeManager
    @Environment(\.modelContext) var modelContext

    var body: some View {
        ZStack(alignment: .bottomTrailing) {
            List {
                ForEach(items, id: \.id) { item in
                    BibEntrySearchResultItemView(item: item)
                }
            }
        }
    }

}

#Preview {
    BibListView(
        items: [],
        editing: .constant(nil),
        editorContainer: BibtexEditorWebviewContainer(bounce: true, scrollEnabled: true)
    )
}
