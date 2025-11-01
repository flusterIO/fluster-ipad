//
//  editor_view.swift
//  Fluster
//
//  Created by Andrew on 10/29/25.
//

import SwiftUI
import STTextViewSwiftUI


struct MarkdownEditorView: View {
    @Binding var editorValue: AttributedString
    @Environment(ThemeManager.self) private var themeManager: ThemeManager
    @Environment(\.colorScheme) var colorScheme: ColorScheme
    var body: some View {
        ZStack {
            themeManager.theme.background
            TextView(text: $editorValue,
                     options: [.showLineNumbers, .wrapLines],
            )
        }
    }
}
