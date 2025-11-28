//
//  no_notes_found_view.swift
//  FlusterSwift
//
//  Created by Andrew on 11/28/25.
//

import SwiftUI

public struct NoNotesFoundView: View {
    @Environment(ThemeManager.self) var themeManager: ThemeManager
    let subtitle: String
    public init(subtitle: String = "No notes matched your query.") {
        self.subtitle = subtitle
    }
    public var body: some View {
        VStack(spacing: 16) {
            ZStack {
                themeManager.theme.primary
                    .frame(width: 64, height: 64)
                    .clipShape(Circle())
                Image(systemName: "magnifyingglass")
                    .imageScale(.large)
                    .foregroundStyle(themeManager.theme.primary_foreground)
            }
            Text("No notes found")
                .font(.title2)
            Text(subtitle)
                .multilineTextAlignment(.center)
        }
        .padding()
    }
}

#Preview {
    NoNotesFoundView()
}
