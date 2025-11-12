//
//  select_note_to_continue_view.swift
//  Fluster
//
//  Created by Andrew on 11/10/25.
//

import SwiftUI

struct SelectNoteToContinueView: View {
    @Environment(ThemeManager.self) private var themeManager: ThemeManager

    var body: some View {
        VStack(spacing: 16) {
            ZStack {
                themeManager.theme.primary
                    .frame(width: 64, height: 64)
                    .clipShape(Circle())
                Image(systemName: "magnifyingglass")
                    .imageScale(.large)
                    .foregroundStyle(themeManager.theme.primary_foreground)
            }
            Text("No note selected")
                .font(.title2)
            Text("Select a note in the search page or create a new note to edit your note here.")
        }
    }
}

#Preview {
    SelectNoteToContinueView()
}
