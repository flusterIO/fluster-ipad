//
//  settings_page_view.swift
//  Fluster
//
//  Created by Andrew on 11/17/25.
//

import FlusterSwift
import SwiftUI
import FlusterData
import ConundrumSwift

struct SettingsPageView: View {
  @AppStorage(AppStorageKeys.theme.rawValue) private var theme: FlusterTheme =
    .fluster
  @AppStorage(AppStorageKeys.editorThemeDark.rawValue) private
    var editorThemeDark: CodeEditorTheme = .dracula
  @AppStorage(AppStorageKeys.editorThemeLight.rawValue) private
    var editorThemeLight: CodeEditorTheme = .githubLight
  @AppStorage(AppStorageKeys.colorScheme.rawValue) private
    var colorSchemeSelection: ColorSchemeSelection = .dark
  @AppStorage(AppStorageKeys.silenceParsingErrors.rawValue) private
    var silenceParsingErrors: Bool = false
    @AppStorage(AppStorageKeys.codeBlockThemeDark.rawValue) private var codeBlockThemeDark: SupportedCodeBlockTheme = .dracula
    @AppStorage(AppStorageKeys.codeBlockThemeLight.rawValue) private var codeBlockThemeLight: SupportedCodeBlockTheme = .onehalflight
    @AppStorage(AppStorageKeys.editorSaveMethod.rawValue) private var editorSaveMethod: EditorSaveMethod = .onChange
  @Environment(ThemeManager.self) private var themeManager: ThemeManager

  var body: some View {
    Form {
      Text("Appearance")
            .font(.headline)
      Section(header: Text("Color Scheme")) {
        ColorSchemePickerView(scheme: $colorSchemeSelection)
      }
      Section(header: Text("Font Size")) {
        FontSizePicker()
      }
        Section(header: Text("Codeblock Theme")) {
            Picker(selection: $codeBlockThemeLight, label: Text("Light Mode")) {
                ForEach(SupportedCodeBlockTheme.allCases, id: \.rawValue) { item in
                    Text(item.toString()).tag(item)
              }
            }
            Picker(selection: $codeBlockThemeDark, label: Text("Dark Mode")) {
                ForEach(SupportedCodeBlockTheme.allCases, id: \.rawValue) { item in
                    Text(item.toString()).tag(item)
              }
            }
        }
      .listRowSeparator(.hidden)
      Text("Editor")
        .font(.headline)
      Section(header: Text("Keymap")) {
        EditorKeymapPickerView()
      }
      .listRowSeparator(.hidden)
        Section(header: Text("Save Method"), footer: Text("Set this to 'on-save' and use cmd+s to save to avoid jittery re-renders.")) {
            Picker(selection: $editorSaveMethod, label: Text("Render Method")) {
                Text("On Save").tag(EditorSaveMethod.onSave)
                Text("On Change").tag(EditorSaveMethod.onChange)
            }
        }
      Section(header: Text("Editor Theme")) {
        EditorThemePickerView(
          theme: $editorThemeDark,
          title: "Dark Mode"
        )
        EditorThemePickerView(
          theme: $editorThemeLight,
          title: "Light Mode"
        )
      }
      Toggle(
        isOn: $silenceParsingErrors,
        label: {
          Text("Silence parsing errors")
        }
      )
      .tint(themeManager.theme.primary)
    }
    .navigationTitle("Settings")
  }
}
