//
//  font_size_picker_view.swift
//  Fluster
//
//  Created by Andrew on 11/21/25.
//

import SwiftUI
import FlusterSwift

struct FontSizePicker: View {

    @AppStorage(AppStorageKeys.webviewFontSize.rawValue) private var webviewFontSize: WebviewFontSize = .base
    @Environment(\.colorScheme) var colorScheme
    @Environment(ThemeManager.self) private var themeManager: ThemeManager

    var body: some View {
        Picker(selection: $webviewFontSize, label: Text("Font Size")) {
            ForEach(0..<WebviewFontSize.allCases.count) {
                Text(WebviewFontSize.allCases[$0].rawValue).tag(
                    WebviewFontSize.allCases[$0]
                )
            }
        }
        .pickerStyle(.segmented)
    }
}
