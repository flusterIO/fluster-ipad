//
//  no_note_selected_view.swift
//  Fluster
//
//  Created by Andrew on 1/19/26.
//

import SwiftUI

struct NoNoteSelectedView: View {
    var body: some View {
        VStack {
            Image(systemName: "plus.magnifyingglass")
                .resizable()
                .frame(width: 32, height: 32)
                .padding()
                .background(Circle().fill(Color.accent))
            Text("No note selected")
                .font(.title)
            Text("Use one of the many search features to select a note to view it's content here. Start by using ⌘-shift-p to open the command palette.")
                .font(.caption)
        }
        .frame(maxWidth: 450)
    }
}

#Preview {
    NoNoteSelectedView()
}
