//
//  code_theme_picker_view.swift
//  Fluster
//
//  Created by Andrew on 11/17/25.
//

import SwiftUI
import FlusterSwift

func getEditorThemeText(editorTheme: CodeSyntaxTheme) -> String {
    switch editorTheme {
    case .githubDark:
        return "Github Dark"
    case .githubLight:
        return "Github Light"
    case .aura:
        return "Aura"
    case .dracula:
        return "Dracula"
    case .materialDark:
        return "Material Dark"
    case .materialLight:
        return "Material Light"
    case .solarizedDark:
        return "Solaraized Dark"
    case .solarizedLight:
        return "Solaraized Light"
    case .tokyoNight:
        return "Tokyo Night"
    case .tokyoNightDay:
        return "Tokyo Night Day"
    case .tokyoNightStorm:
        return "Tokyo Night Storm"
    case .xcodeDark:
        return "Xcode Dark"
    case .xcodeLight:
        return "Xcode Light"
    }
}

struct EditorThemePickerView: View {

    @Binding var theme: CodeSyntaxTheme
    var title: String
    @Environment(\.colorScheme) var colorScheme
    @Environment(ThemeManager.self) private var themeManager: ThemeManager

    var body: some View {
        Picker(selection: $theme, label: Text(title)) {
            ForEach(0..<CodeSyntaxTheme.allCases.count) {
                Text(getEditorThemeText(editorTheme: CodeSyntaxTheme.allCases[$0])).tag(
                    CodeSyntaxTheme.allCases[$0]
                )
            }
        }
    }
}

#Preview {
    EditorThemePickerView(theme: .constant(.githubDark), title: "Test title")
}
