//
//  File.swift
//  FlusterData
//
//  Created by Andrew on 4/29/26.
//

import Foundation
import ConundrumSwift


public extension SupportedCodeBlockTheme {
    func toString() -> String {
        switch self {
        case .thirteenThirtySeven:
            return "1337"
        case .coldarkCold:
            return "Cold Dark: Cold"
        case .coldarkDark:
            return "Cold Dark: Dark"
        case .darkneon:
            return "Dark Neon"
        case .dracula:
            return "Dracula"
        case .github:
            return "Github"
        case .monokaiExtended:
            return "Monokai: Extended"
        case .monokaiExtendedBright:
            return "Monokai: Extended Bright"
        case .monokaiExtendedLight:
            return "Monokai: Extended Light"
        case .monoakaiExtendedOrigin:
            return "Monokai: Extended Origin"
        case .nord:
            return "Nord"
        case .onehalfdark:
            return "One Half: Dark"
        case .onehalflight:
            return "One Half: Light"
        case .solarizedDark:
            return "Solarized: Dark"
        case .solarizedLight:
            return "Solarized: Light"
        case .sublimeSnazzy:
            return "Sublime Snazzy"
        case .twodark:
            return "Two Dark"
        case .visualStudioDarkPlus:
            return "Visual Studio+"
        case .ansi:
            return "Ansi"
        case .base16:
            return "Base16"
        case .base16256:
            return "Base16: 256"
        case .gruvboxDark:
            return "Gruvbox: Dark"
        case .gruvboxLight:
            return "Gruvbox: Light"
        case .zenburn:
            return "Zenburn"
    }
    }
}
