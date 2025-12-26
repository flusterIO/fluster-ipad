import FlusterSwift
import SwiftUI

struct MarkdownTabView: View {
  @Binding var editingNote: NoteModel?
  @AppStorage(AppStorageKeys.editorThemeDark.rawValue) private
    var editorThemeDark: CodeSyntaxTheme = .dracula
  @AppStorage(AppStorageKeys.editorThemeLight.rawValue) private
    var editorThemeLight: CodeSyntaxTheme = .githubLight
  @AppStorage(AppStorageKeys.theme.rawValue) private var theme: WebViewTheme =
    .fluster
  @AppStorage(AppStorageKeys.editorKeymap.rawValue) private var editorKeymap: EditorKeymap = .base
  @AppStorage(AppStorageKeys.hasLaunchedPreviously.rawValue) private
    var hasPreviouslyLaunched: Bool = false
  @Environment(ThemeManager.self) private var themeManager: ThemeManager
  var editorContainer: MdxEditorWebviewContainer
  init(editingNote: Binding<NoteModel?>, editorContainer: MdxEditorWebviewContainer) {
    self._editingNote = editingNote
    self.editorContainer = editorContainer
  }
  var body: some View {
    if let editingNoteBinding = Binding($editingNote) {
      NavigationStack {
        MdxEditorWebview(
          url:
            Bundle.main.url(
              forResource: "index",
              withExtension: "html",
              subdirectory: "splitview_mdx_editor"
            )!,
          theme: $theme,
          editorThemeDark: $editorThemeDark,
          editorThemeLight: $editorThemeLight,
          editingNote: editingNoteBinding,
          editorKeymap: $editorKeymap,
          container: editorContainer,
        )
        // TODO: Remove this. This is just for easy development.
        .onAppear {
          if let parsedMdx = editingNote?.markdown
            .preParsedBody
          {
            editorContainer.setParsedEditorContentString(
              content: parsedMdx
            )
          }
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
