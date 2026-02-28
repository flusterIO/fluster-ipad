import FlusterData
import FlusterSwift
import SwiftUI

struct MarkdownTabView: View {
  @Binding var editingNote: NoteModel?
  @Binding var fullScreenCover: MainFullScreenCover?
  var onNavigateToNote: (NoteModel) -> Void = { _ in }
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
  init(
    editingNote: Binding<NoteModel?>, editorContainer: MdxEditorWebviewContainer,
    onNavigateToNote: ((NoteModel) -> Void)?,
    fullScreenCover: Binding<MainFullScreenCover?>
  ) {
    self._editingNote = editingNote
    self._fullScreenCover = fullScreenCover
    self.editorContainer = editorContainer
    if onNavigateToNote != nil {
      self.onNavigateToNote = onNavigateToNote!
    }
  }
  var body: some View {
    if let editingNoteBinding = Binding($editingNote) {
      NavigationStack {
        MdxEditorWebview(
          url:
            Bundle.main.url(
              forResource: "index_ipad",
              withExtension: "html",
              subdirectory: "splitview_mdx_editor_ipad"
            )!,
          theme: $theme,
          editorThemeDark: $editorThemeDark,
          editorThemeLight: $editorThemeLight,
          editingNote: editingNoteBinding,
          editorKeymap: $editorKeymap,
          container: editorContainer,
          onNavigateToNote: onNavigateToNote,
          fullScreenCover: $fullScreenCover
        )
        .toolbarVisibility(.hidden, for: .navigationBar)
        .frame(maxWidth: .infinity, maxHeight: .infinity)
        //                .frame(width: geo.size.width, height: geo.size.height, alignment: .topLeading)
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
