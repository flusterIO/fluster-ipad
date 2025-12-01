//
//  split_view_editor_view.swift
//  Fluster
//
//  Created by Andrew on 11/28/25.
//

import FlusterSwift
import SwiftUI

struct SplitViewEditorView: View {
    @StateObject private var previewContainer = MdxPreviewWebviewContainer(
        bounce: false,
        scrollEnabled: false
    )
    @State private var shouldShowEditor: Bool = false
    @State private var previewHeight: CGFloat = 0
    @State private var editorHeight: CGFloat = 0
    @State private var isDragging: Bool = true
    @Environment(\.colorScheme) var colorScheme: ColorScheme
    @Binding var theme: WebViewTheme
    @Binding var editorThemeDark: CodeSyntaxTheme
    @Binding var editorThemeLight: CodeSyntaxTheme
    @Binding var editingNote: NoteModel?
    @Binding var editorKeymap: EditorKeymap
    //    @StateObject private var editorContainer = MdxEditorWebviewContainer()
    let editorContainer: MdxEditorWebviewContainer
    init(
        theme: Binding<WebViewTheme>,
        editorThemeDark: Binding<CodeSyntaxTheme>,
        editorThemeLight: Binding<CodeSyntaxTheme>,
        editingNote: Binding<NoteModel?>,
        editorKeymap: Binding<EditorKeymap>,
        editorContainer: MdxEditorWebviewContainer
    ) {
        self._theme = theme
        self._editorThemeDark = editorThemeDark
        self._editorThemeLight = editorThemeLight
        self._editingNote = editingNote
        self._editorKeymap = editorKeymap
        self.editorContainer = editorContainer
    }
    var body: some View {
        GeometryReader { rect in
            SplitView(
                left: {
                    ScrollView {
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
                            viewportHeight: $editorHeight,
                            container: editorContainer,
                        )
                        .frame(height: editorHeight)
                        .padding(.bottom, 0)
                        .ignoresSafeArea(edges: .bottom)
                        .onAppear {
                            editorContainer.requestDocumentSize()
                        }
                    }
                },
                right: {
                    ScrollView {
                        MdxPreviewWebview(
                            url:
                                Bundle.main.url(
                                    forResource: "index",
                                    withExtension: "html",
                                    subdirectory: "standalone_mdx_preview"
                                )!,
                            theme: $theme,
                            editorThemeDark: $editorThemeDark,
                            editorThemeLight: $editorThemeLight,
                            editingNote: $editingNote,
                            editorKeymap: $editorKeymap,
                            shouldShowEditor: $shouldShowEditor,
                            viewportHeight: $previewHeight,
                            container: previewContainer,
                        )
                        .frame(
                            height: previewHeight
                        )
                        .padding(.bottom, 0)
                        .ignoresSafeArea(edges: .bottom)
                    }
                    .frame(
                        height: getPreviewHeight(rect: rect)
                    )
                },
                onDragStart: {
                    isDragging = true
                    previewHeight = self.viewportHeight(rect: rect)
                    previewContainer.setLoading(isLoading: true)
                },
                onDragEnd: {
                    //                isDragging = false
                    // TODO: Close one side animation if ratio below or above certain bounds. Use a state variabl enum for this though and make sure to persist it in appstorage.
                    //                    withAnimation(.easeInOut) {
                    //                        self.editorHeight = 0
                    //                        self.previewHeight = 0
                    //                    }
                    previewContainer.setLoading(isLoading: false)
                    previewContainer.requestDocumentSize()
                    editorContainer.requestDocumentSize()
                }
            )
            .onChange(
                of: editingNote,
                {
                    if let note = editingNote {
                        editorContainer.setInitialContent(
                            note: note
                        )
                        previewContainer.setInitialContent(
                            note: note
                        )
                        editorContainer.resetScrollPosition()
                    }
                }
            )
            .onChange(
                of: editorThemeDark,
                {
                    editorContainer.emitEditorThemeEvent(
                        theme: colorScheme == .dark
                            ? editorThemeDark : editorThemeLight
                    )
                    editorContainer.setEditorDarkTheme(
                        theme: editorThemeDark
                    )
                }
            )
            .onChange(
                of: editorThemeLight,
                {
                    editorContainer.emitEditorThemeEvent(
                        theme: colorScheme == .dark
                            ? editorThemeDark : editorThemeLight
                    )
                    editorContainer.setEditorLightTheme(
                        theme: editorThemeLight
                    )
                }
            )
            .onChange(
                of: editorKeymap,
                {
                    editorContainer.setEditorKeymap(
                        editorKeymap: editorKeymap
                    )
                }
            )
            .disableAnimations()
        }
    }
    func viewportHeight(rect: GeometryProxy) -> CGFloat {
        print("Getting height")
        if let h = UIScreen.current?.bounds.height {
            return h - rect.safeAreaInsets.top
        } else {
            return 0
        }
    }
    func getPreviewHeight(rect: GeometryProxy) -> CGFloat {
        if !isDragging {
            return previewHeight
        }
        return viewportHeight(rect: rect)
    }
}
