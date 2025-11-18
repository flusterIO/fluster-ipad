//
//  theme_picker_view.swift
//  Fluster
//
//  Created by Andrew on 11/17/25.
//

import SwiftUI

struct ThemePickerView: View {

    @Binding var theme: WebViewTheme
    @Environment(ThemeManager.self) private var themeManager: ThemeManager
    @Environment(\.colorScheme) var colorScheme

    var body: some View {
        Picker(selection: $theme, label: Text("App Theme")) {
            ForEach(0..<WebViewTheme.allCases.count) {
                Text(WebViewTheme.allCases[$0].rawValue).tag(
                    WebViewTheme.allCases[$0]
                )
            }
        }
//        .tint(colorScheme == .dark ? .white : .black)
    }
}

#Preview {
    ThemePickerView(theme: .constant(.fluster))
}
