//
//  note_search_result_item_view.swift
//  Fluster
//
//  Created by Andrew on 11/7/25.
//

import SwiftUI
import PencilKit

struct NoteSearchResultItemView: View {
    var item: NoteModel
    var body: some View {
        Text(item.markdown.title ?? "--")
            .font(.title)
    }
}

#Preview {
    NoteSearchResultItemView(item: NoteModel(drawing: PKDrawing.init().dataRepresentation(), markdown: MarkdownNote(body: "# My Note title")))
}
