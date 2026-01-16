import FlusterSwift
import PencilKit
import SwiftUI
import FlusterData

struct PaperTabView: View {
  @State private var toolbar: PKToolPicker = PKToolPicker()
  @Environment(ThemeManager.self) private var themeManager: ThemeManager
  @AppStorage(AppStorageKeys.hasLaunchedPreviously.rawValue) private
    var hasPreviouslyLaunched: Bool = false
  @Binding var editingNote: NoteModel?
  @Binding var selectedTab: IpadMainViewTab
  init(
    editingNote: Binding<NoteModel?>,
    selectedTab: Binding<IpadMainViewTab>
  ) {
    self._editingNote = editingNote
    self._selectedTab = selectedTab
  }
  var body: some View {
    if let unwrappedEditingNote = Binding($editingNote) {
      PaperView(
        toolbar: $toolbar,
        drawingData: unwrappedEditingNote.drawing,
        activeTab: $selectedTab
      )
      .backgroundExtensionEffect()
      .ignoresSafeArea()
    } else {
      if hasPreviouslyLaunched {
        SelectNoteToContinueView()
      } else {
        ProgressView()
          .progressViewStyle(.circular)
          .scaleEffect(1.5)
          .tint(themeManager.theme.primary)
      }
    }
  }
}
