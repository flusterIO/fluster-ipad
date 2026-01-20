//
//  view_mdx_note.swift
//  Fluster
//
//  Created by Andrew on 1/19/26.
//

import SwiftUI
import FlusterData

struct ViewMdxNoteView: View {
    let item: NoteModel
    var body: some View {
        Text("View Mdx Note")
    }
}

#Preview {
    ViewMdxNoteView(item: NoteModel.fromNoteBody(noteBody: "# My Note"))
}
