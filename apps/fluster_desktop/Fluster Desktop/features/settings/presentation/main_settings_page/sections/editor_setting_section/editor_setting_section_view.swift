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
        VStack(alignment: .leading, spacing: 8) {
          Text("Keymap").font(.headline)
          Picker(
            selection: $editorKeymap,
            content: {
              Text("Basic").tag(EditorKeymap.base)
              Text("Vim").tag(EditorKeymap.vim)
              Text("Emacs").tag(EditorKeymap.emacs)
            },
            label: {
              Text("Keymap Scheme")
            }
          )
          .pickerStyle(.segmented)
          Text("Theme").font(.headline)
            .padding(.top)
          Picker(
            selection: $editorThemeLight,
            content: {
              ForEach(CodeSyntaxTheme.allCases, id: \.rawValue) { item in
                Text(item.toThemeLabel()).tag(item)
              }
            },
            label: {
              Text("Light")
            }
          )
          Picker(
            selection: $editorThemeDark,
            content: {
              ForEach(CodeSyntaxTheme.allCases, id: \.rawValue) { item in
                Text(item.toThemeLabel()).tag(item)
              }
            },
            label: {
              Text("Dark")
            }
          )
        }
        .frame(maxWidth: .infinity, alignment: .leading)
      })
  }
}

#Preview {
  EditorSettingSectionView()
}
