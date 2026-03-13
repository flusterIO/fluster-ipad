import FlusterData
import FlusterSwift
import SwiftUI

struct MarkdownTabView: View {
  @Binding var editingNote: NoteModel?
  @Binding var fullScreenCover: MainFullScreenCover?
  var onNavigateToNote: (NoteModel) -> Void = { _ in }
  @AppStorage(AppStorageKeys.editorThemeDark.rawValue) private
    var editorThemeDark: CodeEditorTheme = .dracula
  @AppStorage(AppStorageKeys.editorThemeLight.rawValue) private
    var editorThemeLight: CodeEditorTheme = .githubLight
  @AppStorage(AppStorageKeys.theme.rawValue) private var theme: FlusterTheme =
    .fluster
  @AppStorage(AppStorageKeys.editorKeymap.rawValue) private var editorKeymap: CodeEditorKeymap =
    .base
  @AppStorage(AppStorageKeys.hasLaunchedPreviously.rawValue) private
    var hasPreviouslyLaunched: Bool = false
  @Environment(ThemeManager.self) private var themeManager: ThemeManager
  init(
    editingNote: Binding<NoteModel?>,
    onNavigateToNote: ((NoteModel) -> Void)?,
    fullScreenCover: Binding<MainFullScreenCover?>
  ) {
    self._editingNote = editingNote
    self._fullScreenCover = fullScreenCover
    if onNavigateToNote != nil {
      self.onNavigateToNote = onNavigateToNote!
    }
  }
  var body: some View {
    if let editingNoteBinding = Binding($editingNote) {
      NavigationStack {
        MdxEditorWebview(
          editingNote: editingNoteBinding,
          fullScreenCover: $fullScreenCover,
          onNavigateToNote: { _ in
          }
        )
        //        .toolbarVisibility(.hidden, for: .navigationBar)
        //        .frame(maxWidth: .infinity, maxHeight: .infinity)
        // TODO: Remove this. This is just for easy development.
        .onAppear {
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
