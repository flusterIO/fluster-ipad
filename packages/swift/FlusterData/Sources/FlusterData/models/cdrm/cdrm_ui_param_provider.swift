//
//  File.swift
//  FlusterData
//
//  Created by Andrew on 6/12/26.
//

import Foundation
import SwiftUI
import ConundrumSwift


public final class UIParamsProvider {
    @AppStorage(AppStorageKeys.codeBlockThemeDark.rawValue) public var codeBlockThemeDark:
      SupportedCodeBlockTheme = .dracula
    @AppStorage(AppStorageKeys.codeBlockThemeLight.rawValue)public  var codeBlockThemeLight:
      SupportedCodeBlockTheme = .monokaiExtendedLight
    @AppStorage(AppStorageKeys.webviewFontScale.rawValue) public  var webviewFontScale: Double = 1
    @AppStorage(AppStorageKeys.webviewMathFontScale.rawValue) public var webviewMathFontScale: Double = 1.2
    @AppStorage(AppStorageKeys.codeBlockThemeDark.rawValue) public var codeblockThemeDark: SupportedCodeBlockTheme = .dracula
    @AppStorage(AppStorageKeys.codeBlockThemeLight.rawValue) public var codeblockThemeLight: SupportedCodeBlockTheme = .solarizedLight

    public init() {
        
    }

    
    public func toParams(colorScheme: ColorScheme) -> UiParams {
        UiParams(darkMode: colorScheme == .dark, fontScalar: Float(webviewFontScale), mathFontScalar: Float(webviewMathFontScale), syntaxTheme: colorScheme == .dark ? codeBlockThemeDark : codeBlockThemeLight)
    }
}
