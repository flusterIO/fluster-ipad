//
//  settings_page_view.swift
//  Fluster
//
//  Created by Andrew on 11/17/25.
//

import FlusterSwift
import SwiftUI

struct SettingsPageView: View {
    @AppStorage(AppStorageKeys.theme.rawValue) private var theme: WebViewTheme =
        .fluster
    @AppStorage(AppStorageKeys.editorThemeDark.rawValue) private
        var editorThemeDark: CodeSyntaxTheme = .dracula
    @AppStorage(AppStorageKeys.editorThemeLight.rawValue) private
        var editorThemeLight: CodeSyntaxTheme = .githubLight
    @AppStorage(AppStorageKeys.colorScheme.rawValue) private
        var colorSchemeSelection: ColorSchemeSelection = .dark
    @AppStorage(AppStorageKeys.silenceParsingErrors.rawValue) private
        var silenceParsingErrors: Bool = false
    @Environment(ThemeManager.self) private var themeManager: ThemeManager

    var body: some View {
        Form {
            Section(header: Text("Appearance")) {
                ColorSchemePickerView(scheme: $colorSchemeSelection)
                ThemePickerView(theme: $theme)
            }
            Section(header: Text("Font Size")) {
                FontSizePicker()
            }
            .listRowSeparator(.hidden)
            Text("Editor")
                .font(.title)
            Section(header: Text("Keymap")) {
                EditorKeymapPickerView()
            }
            .listRowSeparator(.hidden)
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
            Toggle(isOn: $silenceParsingErrors, label: {
                Text("Silence parsing errors")
            })
            .tint(themeManager.theme.primary)
        }
        .navigationTitle("Settings")
    }
}
