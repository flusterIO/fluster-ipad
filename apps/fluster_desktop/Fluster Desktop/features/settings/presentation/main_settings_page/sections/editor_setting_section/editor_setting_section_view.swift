//
//  editor_settng_section_view.swift
//  Fluster
//
//  Created by Andrew on 1/20/26.
//

import FlusterData
import SwiftUI

struct EditorSettingSectionView: View {
  @AppStorage(AppStorageKeys.editorKeymap.rawValue) private var editorKeymap: EditorKeymap = .base
    @AppStorage(AppStorageKeys.editorThemeDark.rawValue) private var editorThemeDark:
      CodeSyntaxTheme = .dracula
    @AppStorage(AppStorageKeys.editorThemeLight.rawValue) private var editorThemeLight:
      CodeSyntaxTheme = .materialLight

  var body: some View {
    SettingsSection(
      title: "Editor", subtitle: nil,
      content: {
        VStack(alignment: .leading) {
          Picker(
            selection: $editorKeymap,
            content: {
              Text("Basic").tag(EditorKeymap.base)
              Text("Vim").tag(EditorKeymap.vim)
              Text("Emacs").tag(EditorKeymap.emacs)
            },
            label: {
              Text("Editor Keymap")
            }
          )
          .pickerStyle(.segmented)
        }
        .frame(maxWidth: .infinity, alignment: .leading)
          
      })
  }
}

#Preview {
  EditorSettingSectionView()
}
