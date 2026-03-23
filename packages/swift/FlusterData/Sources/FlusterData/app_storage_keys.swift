//
//  app_storage_keys.swift
//  Fluster
//
//  Created by Andrew on 11/17/25.
//

public enum AppStorageKeys: String {
  case theme,
    editorThemeDark,
    editorThemeLight,
    colorScheme,
    editorKeymap,
    webviewFontSize,
    tabviewCustomization,
    hasLaunchedPreviously,
    splitViewEditorSplit,
    silenceParsingErrors,
    lockEditorScrollToPreview,
    embeddedCslFile,
    
    editorSaveMethod,
    // Snippets
    includeEmojiSnippets,
    defaultNoteView,
    navigationColVisibility,
    /// The notesDirectory is a string that is initially empty. This value must always be checked for it's empty status instead of a null value.
    notesDirectory,
    /// Defaults to true.
    respectGitIgnore,
    // -- UI State --
    noteSidebarSectionOpen,
    flusterSidebarSectionOpen,
    desktopWebviewTheme
    /// A user name used only for the AI to personalize the application.
    case userPreferredName
    /// The global search function selected on the mac, allowing seperate settings if data ever overlaps
    case globalSearchFunctionMac
    /// The global search function selected on the iPad, allowing seperate settings if data ever overlaps
    case globalSearchFunctionIpad
}
