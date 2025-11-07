//
//  markdown_preview_view.swift
//  Fluster
//
//  Created by Andrew on 11/1/25.
//

import SwiftUI
import MarkdownUI


struct MarkdownPreviewView: View {
    @Binding var content: AttributedString
    @Environment(ThemeManager.self) private var themeManager: ThemeManager

    var body: some View {
            Markdown {
                String(content.characters)
            }
            .markdownBlockStyle(\.blockquote) { configuration in
              configuration.label
                .padding()
                .markdownTextStyle {
                  FontWeight(.semibold)
                  BackgroundColor(nil)
                }
                .overlay(alignment: .leading) {
                  Rectangle()
                        .fill(themeManager.theme.primary)
                    .frame(width: 4)
                }
            }
            .padding()
            .markdownTheme(.gitHub)
    }
}

#Preview {
    MarkdownPreviewView(content: .constant("Hello World"))
}
