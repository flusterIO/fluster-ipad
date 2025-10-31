import SwiftUI

public protocol ThemeProtocol {
    var background: Color { get }
    var foreground: Color { get }
    var card: Color { get }
    var card_foreground: Color { get }
    var popover: Color { get }
    var popover_foreground: Color { get }
    var primary: Color { get }
    var primary_foreground: Color { get }
    var secondary: Color { get }
    var secondary_foreground: Color { get }
    var muted: Color { get }
    var muted_foreground: Color { get }
    var accent: Color { get }
    var accent_foreground: Color { get }
    var destructive: Color { get }
    var destructive_foreground: Color { get }
    var border: Color { get }
    var input: Color { get }
    var ring: Color { get }
    var general_link_color: Color { get }
}

