//
//  note_search_result_item_view.swift
//  Fluster
//
//  Created by Andrew on 11/7/25.
//

import PencilKit
import SwiftData
import SwiftUI

public struct NoteSearchResultItemInnerView: View {
    @Binding var editingNote: NoteModel?
    public let item: NoteModel
    @Environment(ThemeManager.self) private var themeManager: ThemeManager
    @Environment(\.dismiss) private var dismiss
    public var body: some View {
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
                    .font(.caption)
            }
        }
        .onTapGesture {
            editingNote = item
            dismiss()
        }
    }
}

public struct NoteSearchResultItemView: View {
    public var item: NoteModel
    @Binding var editingNote: NoteModel?
    @State private var confirmationDeleteModalOpen: Bool = false
    @Environment(ThemeManager.self) private var themeManager: ThemeManager
    @Environment(\.modelContext) var modelContext

    public var body: some View {
        NoteSearchResultItemInnerView(editingNote: $editingNote, item: item)
            .swipeActions(allowsFullSwipe: true) {
                Button(
                    action: {
                        confirmationDeleteModalOpen = true
                    },
                    label: {
                        Label("Delete", systemImage: "trash")
                    }
                )
            }
            .confirmationDialog(
                "Are you sure you want to remove this note?",
                isPresented: $confirmationDeleteModalOpen,
                actions: {
                    Button("Clear", role: .cancel) {
                        //                    confirmationDeleteModalOpen = false
                    }
                    Button("Delete") {
                        modelContext.delete(item)
                        confirmationDeleteModalOpen = false
                    }
                },
                message: {
                    Text("Are you sure you want to delete this note?")
                }
            )
    }
}

#Preview {
    NoteSearchResultItemView(
        item: NoteModel(
            drawing: PKDrawing.init().dataRepresentation(),
            markdown: MarkdownNote(body: "# My Note title", summary: nil)
        ),
        editingNote: .constant(nil)
    )
}
