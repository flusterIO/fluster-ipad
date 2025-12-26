import FlusterSwift
import SwiftUI

struct NoteDetailsTabView: View {
  @AppStorage(AppStorageKeys.hasLaunchedPreviously.rawValue) private var hasPreviouslyLaunched:
    Bool = false
  @Environment(ThemeManager.self) private var themeManager: ThemeManager
  @Binding var editingNote: NoteModel?
  @Binding var fullScreenCover: MainFullScreenCover?

  init(editingNote: Binding<NoteModel?>, fullScreenCover: Binding<MainFullScreenCover?>) {
    self._editingNote = editingNote
    self._fullScreenCover = fullScreenCover
  }
  var body: some View {
    if let noteBinding = Binding($editingNote) {
      NavigationStack {
        NoteDetailWebview(
          fullScreenCover: $fullScreenCover,
          note: noteBinding,
        )
        .onAppear {  // hack to avoid scroll bouncing with awkward colored background.
          UIScrollView.appearance().bounces = false
          UIScrollView.appearance().isScrollEnabled =
            false
        }
        .onDisappear {
          UIScrollView.appearance().bounces = true
          UIScrollView.appearance().isScrollEnabled = true
        }
      }
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
