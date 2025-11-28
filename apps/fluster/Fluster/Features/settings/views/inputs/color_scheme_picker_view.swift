//
//  color_scheme_picker_view.swift
//  Fluster
//
//  Created by Andrew on 11/17/25.
//

import SwiftUI
import FlusterSwift

struct ColorSchemePickerView: View {

    @Binding var scheme: ColorSchemeSelection
    @Environment(\.colorScheme) var colorScheme
    @Environment(ThemeManager.self) private var themeManager: ThemeManager

    var body: some View {
        Picker(selection: $scheme, label: Text("Editor Theme")) {
            ForEach(0..<ColorSchemeSelection.allCases.count) {
                Text(ColorSchemeSelection.allCases[$0].rawValue).tag(
                    ColorSchemeSelection.allCases[$0]
                )
            }
        }
        .pickerStyle(.segmented)
    }
}

#Preview {
    EditorThemePickerView(theme: .constant(.githubDark), title: "Test title")
}
