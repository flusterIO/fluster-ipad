//
//  settings_page_view.swift
//  Fluster
//
//  Created by Andrew on 11/17/25.
//

import SwiftUI

struct SettingsPageView: View {
    
    @Binding var theme: WebViewTheme
    @Binding var editorTheme: CodeEditorTheme
    @Binding var colorSchemeSelection: ColorSchemeSelection
    @Environment(ThemeManager.self) private var themeManager: ThemeManager

    var body: some View {
        Form {
            Section(header: Text("Appearance")) {
                ColorSchemePickerView(scheme: $colorSchemeSelection)
                ThemePickerView(theme: $theme)
                EditorThemePickerView(theme: $editorTheme)
            }
        }
    }
}

#Preview {
    SettingsPageView(theme: .constant(.fluster), editorTheme: .constant(.solarizedLight), colorSchemeSelection: .constant(.dark))
}
