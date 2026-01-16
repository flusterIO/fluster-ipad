import FlusterSwift
import SwiftUI
import FlusterData

struct BibliographyTabView: View {
  @Binding var editingNote: NoteModel?
  init(editingNote: Binding<NoteModel?>) {
    self._editingNote = editingNote
  }
  var body: some View {
    NavigationStack {
      BibliographyPageView(
        editingNote: $editingNote
      )
    }
  }
}
