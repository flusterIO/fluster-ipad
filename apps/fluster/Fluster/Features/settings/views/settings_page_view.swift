//
//  settings_page_view.swift
//  Fluster
//
//  Created by Andrew on 11/17/25.
//

import SwiftUI

struct SettingsPageView: View {

    @Binding var theme: WebViewTheme
    @Binding var editorThemeDark: CodeEditorTheme
    @Binding var editorThemeLight: CodeEditorTheme
    @Binding var colorSchemeSelection: ColorSchemeSelection
    @Environment(ThemeManager.self) private var themeManager: ThemeManager

    var body: some View {
        Form {
            Section(header: Text("Appearance")) {
                ColorSchemePickerView(scheme: $colorSchemeSelection)
                ThemePickerView(theme: $theme)
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
            }
        }
    }
}

#Preview {
    SettingsPageView(
        theme: .constant(.fluster),
        editorThemeDark: .constant(.dracula),
        editorThemeLight: .constant(.solarizedLight),
        colorSchemeSelection: .constant(.dark)
    )
}
