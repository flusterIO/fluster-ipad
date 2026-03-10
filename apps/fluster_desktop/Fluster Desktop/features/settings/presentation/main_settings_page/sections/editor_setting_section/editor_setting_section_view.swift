//
//  editor_settng_section_view.swift
//  Fluster
//
//  Created by Andrew on 1/20/26.
//

import FlusterData
import SwiftUI

struct EditorSettingSectionView: View {
  @AppStorage(AppStorageKeys.editorKeymap.rawValue) private var editorKeymap: CodeEditorKeymap =
    .base
  @AppStorage(AppStorageKeys.editorThemeDark.rawValue) private var editorThemeDark:
    CodeEditorTheme = .dracula
  @AppStorage(AppStorageKeys.editorThemeLight.rawValue) private var editorThemeLight:
    CodeEditorTheme = .materialLight
  @AppStorage(AppStorageKeys.lockEditorScrollToPreview.rawValue) private var lockEditorScroll:
    Bool = false
  @AppStorage(AppStorageKeys.editorSaveMethod.rawValue) private var saveMethod: EditorSaveMethod =
    .onChange

  var body: some View {
    SettingsSection(
      title: "Editor", subtitle: nil,
      content: {
        VStack(alignment: .leading, spacing: 8) {
          Text("Keymap").font(.headline)
          Picker(
            selection: $editorKeymap,
            content: {
              Text("Basic").tag(CodeEditorKeymap.base)
              Text("Vim").tag(CodeEditorKeymap.vim)
              Text("Emacs").tag(CodeEditorKeymap.emacs)
            },
            label: {
              Text("Keymap Scheme")
            }
          )
          .labelsHidden()
          .pickerStyle(.segmented)
          Text("Save Method").font(.headline)
          Picker(
            selection: $saveMethod,
            content: {
              Text("Change").tag(EditorSaveMethod.onChange)
              Text("Save").tag(EditorSaveMethod.onSave)
            },
            label: {
            }
          )
          .labelsHidden()
          .pickerStyle(.segmented)
          Text(
            "Use ⌘+S to save manually. Automatic save may become to jittery when notes contain nested math or other content that requires a second pass."
          )
          .foregroundStyle(.secondary)
          .font(.caption)
          Text("Theme").font(.headline)
            .padding(.top)
          Picker(
            selection: $editorThemeLight,
            content: {
              ForEach(CodeEditorTheme.allCases, id: \.rawValue) { item in
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
              ForEach(CodeEditorTheme.allCases, id: \.rawValue) { item in
                Text(item.toThemeLabel()).tag(item)
              }
            },
            label: {
              Text("Dark")
            }
          )
          SwitchGroup(
            isOn: $lockEditorScroll, title: "Lock Editor Scroll",
            desc:
              "Lock the editor scroll to automatically scroll the preview portionally to the scroll position of the editor. This calculates position based on percentage so estimates may not be perfect. This feature is still finicky, but will be improved significantly once the app is released or I'm less homeless."
          )
          .padding(.top)
        }
        .frame(maxWidth: .infinity, alignment: .leading)
      })
  }
}

#Preview {
  EditorSettingSectionView()
}
