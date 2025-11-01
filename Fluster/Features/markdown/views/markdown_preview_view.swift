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
    var body: some View {
        HStack(alignment: .top) {
            Markdown {
                String(content.characters)
            }
            Spacer()
        }
        .padding()
    }
}

#Preview {
    MarkdownPreviewView(content: .constant("Hello World"))
}
