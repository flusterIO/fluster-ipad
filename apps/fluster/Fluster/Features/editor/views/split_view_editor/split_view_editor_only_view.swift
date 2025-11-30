//
//  split_view_editor_only_view.swift
//  Fluster
//
//  Created by Andrew on 11/28/25.
//

import FlusterSwift
import SwiftUI

struct SplitViewEditorOnlyView: View {
    @Binding var theme: WebViewTheme
    @Binding var editorThemeDark: CodeSyntaxTheme
    @Binding var editorThemeLight: CodeSyntaxTheme
    @Binding var editingNote: NoteModel?
    @Binding var editorKeymap: EditorKeymap
    @Binding var viewportHeight: CGFloat
    let editorContainer: MdxEditorWebviewContainer
    var body: some View {
        MdxEditorWebview(
            url:
                Bundle.main.url(
                    forResource: "index",
                    withExtension: "html",
                    subdirectory: "standalone_mdx_editor"
                )!,
            theme: $theme,
            editorThemeDark: $editorThemeDark,
            editorThemeLight: $editorThemeLight,
            editingNote: $editingNote,
            editorKeymap: $editorKeymap,
            viewportHeight: $viewportHeight,
            container: editorContainer,
        )
    }
}
