//
//  note_search_result_item_view.swift
//  Fluster
//
//  Created by Andrew on 11/7/25.
//

import PencilKit
import SwiftUI

struct NoteSearchResultItemView: View {
    var item: NoteModel
    @Binding var editingNote: NoteModel?
    @Environment(ThemeManager.self) private var themeManager: ThemeManager

    var body: some View {
        HStack {
            RoundedRectangle(cornerRadius: 8)
                .background(
                    editingNote?.id == item.id
                        ? themeManager.theme.primary : Color.clear
                )
                .foregroundStyle(
                    editingNote?.id == item.id
                        ? themeManager.theme.primary : Color.clear
                )
                .frame(width: 4)
                .font(.subheadline)
                .opacity(editingNote?.id == item.id ? 1 : 0)
            VStack(alignment: .leading) {
                Text(item.markdown.title ?? "--")
                    .font(.headline)
                    .lineLimit(2)
                Text(item.ctime.formatted(date: .complete, time: .shortened))
                    .font(.subheadline)
            }
        }
    }
}

#Preview {
    NoteSearchResultItemView(
        item: NoteModel(
            drawing: PKDrawing.init().dataRepresentation(),
            markdown: MarkdownNote(body: "# My Note title")
        ),
        editingNote: .constant(nil)
    )
}
