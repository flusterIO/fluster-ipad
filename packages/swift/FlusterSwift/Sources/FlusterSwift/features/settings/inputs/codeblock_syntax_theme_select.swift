//
//  File.swift
//  FlusterSwift
//
//  Created by Andrew on 4/29/26.
//

import SwiftUI
import FlusterData
import ConundrumSwift

public struct CodeBlockThemePickerLight: View {
    @AppStorage(AppStorageKeys.codeBlockThemeLight.rawValue) private var theme: SupportedCodeBlockTheme = .monokaiExtendedLight
    public init() {
    }
    public var body: some View {
        VStack(alignment: .leading) {
            Text("Code block theme (light mode)")
                .font(.headline)
            Picker(selection: $theme, content: {
                 ForEach(SupportedCodeBlockTheme.allCases, id: \.self) { theme in
                    Text(theme.rawValue)
                }
            }, label: {
                Label(title: {
                    Text("Codeblock theme (light mode)")
                }, icon: {
                    Image(systemName: "paintpalette" )
                })
            })
            .labelsHidden()
        }
    }
}

public struct CodeBlockThemePickerDark: View {
    @AppStorage(AppStorageKeys.codeBlockThemeDark.rawValue) private var theme: SupportedCodeBlockTheme = .darkneon
    public init() {
    }
    public var body: some View {
        VStack(alignment: .leading) {
            Text("Code block theme (dark mode)")
                .font(.headline)
            Picker(selection: $theme, content: {
                 ForEach(SupportedCodeBlockTheme.allCases, id: \.self) { theme in
                    Text(theme.rawValue)
                }
            }, label: {
                Label(title: {
                    Text("Codeblock theme (dark mode)")
                }, icon: {
                    Image(systemName: "paintpalette" )
                })
            })
            .labelsHidden()
        }
    }
}

#Preview {
    CodeBlockThemePickerDark()
}
