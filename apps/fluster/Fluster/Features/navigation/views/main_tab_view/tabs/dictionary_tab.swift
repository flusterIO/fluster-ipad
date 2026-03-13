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
  init(editingNote: Binding<NoteModel?>) {
    self._editingNote = editingNote
  }
  var body: some View {
    DictionaryWebview(editingNote: $editingNote)
  }
}

#Preview {
  DictionaryTab(editingNote: .constant(nil))
}
