//
//  dictionary_tab.swift
//  Fluster
//
//  Created by Andrew on 12/25/25.
//

import FlusterData
import FlusterSwift
import SwiftUI

struct DictionaryTab: View {
  @Binding public var editingNote: NoteModel?
  let container: DictionaryWebviewContainer
  init(editingNote: Binding<NoteModel?>) {
    self._editingNote = editingNote
    self.container = DictionaryWebviewContainer(
      bounce: false, scrollEnabled: false, onLoad: nil, editingNote: editingNote,
      implementation: WebviewImplementation.dictionary)
  }
  var body: some View {
    DictionaryWebview(container: container)
  }
}

#Preview {
  DictionaryTab(editingNote: .constant(nil))
}
