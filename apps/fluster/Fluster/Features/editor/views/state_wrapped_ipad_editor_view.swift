//
//  state_wrapped_ipad_editor_view.swift
//  Fluster
//
//  Created by Andrew on 3/12/26.
//

import FlusterData
import FlusterSwift
import SwiftUI

struct StateWrappedIpadEditorView: View {
  @Binding public var editingNote: NoteModel?
  @Binding public var fullScreenCover: MainFullScreenCover?
  @StateObject private var editorContainer: MdxEditorWebviewContainer
  init(editingNote: Binding<NoteModel?>, fullScreenCover: Binding<MainFullScreenCover?>) {
    self._editingNote = editingNote
    self._fullScreenCover = fullScreenCover
    self._editorContainer = StateObject(
      wrappedValue: MdxEditorWebviewContainer(
        bounce: false,
        scrollEnabled: true,
        onLoad: nil,
        editingNote: editingNote,
        implementation: WebviewImplementation.mdxEditor
      ))
  }
  var body: some View {
    MarkdownTabView(
      editingNote: $editingNote,
      editorContainer: editorContainer,
      onNavigateToNote: nil,
      fullScreenCover: $fullScreenCover,
    )
  }
}

#Preview {
  StateWrappedIpadEditorView(editingNote: .constant(nil), fullScreenCover: .constant(nil))
}
