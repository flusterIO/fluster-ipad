import SwiftUI

// -- Dark Themes --

public struct ZincDark: ThemeProtocol {
  public typealias ItemType = WebViewTheme
  public var id: WebViewTheme = .zinc
  public init() {}
  public var background: Color {
    return Color(red: 0.051, green: 0.051, blue: 0.059)
  }
  public var foreground: Color {
    return Color(red: 0.98, green: 0.98, blue: 0.98)
  }
  public var card: Color {
    return Color(red: 0.051, green: 0.051, blue: 0.059)
  }
  public var card_foreground: Color {
    return Color(red: 0.98, green: 0.98, blue: 0.98)
  }
  public var popover: Color {
    return Color(red: 0.051, green: 0.051, blue: 0.059)
  }
  public var popover_foreground: Color {
    return Color(red: 0.98, green: 0.98, blue: 0.98)
  }
  public var primary: Color {
    return Color(red: 0.98, green: 0.98, blue: 0.98)
  }
  public var primary_foreground: Color {
    return Color(red: 0.094, green: 0.094, blue: 0.098)
  }
  public var secondary: Color {
    return Color(red: 0.153, green: 0.153, blue: 0.165)
  }
  public var secondary_foreground: Color {
    return Color(red: 0.98, green: 0.98, blue: 0.98)
  }
  public var muted: Color {
    return Color(red: 0.153, green: 0.153, blue: 0.165)
  }
  public var muted_foreground: Color {
    return Color(red: 0.631, green: 0.631, blue: 0.667)
  }
  public var accent: Color {
    return Color(red: 0.153, green: 0.153, blue: 0.165)
  }
  public var accent_foreground: Color {
    return Color(red: 0.98, green: 0.98, blue: 0.98)
  }
  public var destructive: Color {
    return Color(red: 0.498, green: 0.114, blue: 0.114)
  }
  public var destructive_foreground: Color {
    return Color(red: 0.98, green: 0.98, blue: 0.98)
  }
  public var border: Color {
    return Color(red: 0.153, green: 0.153, blue: 0.165)
  }
  public var input: Color {
    return Color(red: 0.153, green: 0.153, blue: 0.165)
  }
  public var ring: Color {
    return Color(red: 0.831, green: 0.831, blue: 0.847)
  }
  public var general_link_color: Color {
    return Color(red: 0.118, green: 0.565, blue: 1)
  }
}

public struct YellowDark: ThemeProtocol {
  public typealias ItemType = WebViewTheme
  public var id: WebViewTheme = .yellow
  public init() {}
  public var background: Color {
    return Color(red: 0.102, green: 0.086, blue: 0.059)
  }
  public var foreground: Color {
    return Color(red: 0.98, green: 0.98, blue: 0.976)
  }
  public var card: Color {
    return Color(red: 0.102, green: 0.086, blue: 0.059)
  }
  public var card_foreground: Color {
    return Color(red: 0.98, green: 0.98, blue: 0.976)
  }
  public var popover: Color {
    return Color(red: 0.102, green: 0.086, blue: 0.059)
  }
  public var popover_foreground: Color {
    return Color(red: 0.98, green: 0.98, blue: 0.976)
  }
  public var primary: Color {
    return Color(red: 0.98, green: 0.8, blue: 0.078)
  }
  public var primary_foreground: Color {
    return Color(red: 0.2, green: 0.137, blue: 0.02)
  }
  public var secondary: Color {
    return Color(red: 0.145, green: 0.122, blue: 0.098)
  }
  public var secondary_foreground: Color {
    return Color(red: 0.98, green: 0.98, blue: 0.976)
  }
  public var muted: Color {
    return Color(red: 0.145, green: 0.122, blue: 0.098)
  }
  public var muted_foreground: Color {
    return Color(red: 0.647, green: 0.62, blue: 0.573)
  }
  public var accent: Color {
    return Color(red: 0.145, green: 0.122, blue: 0.098)
  }
  public var accent_foreground: Color {
    return Color(red: 0.98, green: 0.98, blue: 0.976)
  }
  public var destructive: Color {
    return Color(red: 0.498, green: 0.114, blue: 0.114)
  }
  public var destructive_foreground: Color {
    return Color(red: 0.98, green: 0.98, blue: 0.976)
  }
  public var border: Color {
    return Color(red: 0.145, green: 0.122, blue: 0.098)
  }
  public var input: Color {
    return Color(red: 0.145, green: 0.122, blue: 0.098)
  }
  public var ring: Color {
    return Color(red: 0.898, green: 0.678, blue: 0.169)
  }
  public var general_link_color: Color {
    return Color(red: 0.118, green: 0.565, blue: 1)
  }
}

public struct VioletDark: ThemeProtocol {
  public typealias ItemType = WebViewTheme
  public var id: WebViewTheme = .violet
  public init() {}
  public var background: Color {
    return Color(red: 0.082, green: 0.016, blue: 0.18)
  }
  public var foreground: Color {
    return Color(red: 0.969, green: 0.961, blue: 0.988)
  }
  public var card: Color {
    return Color(red: 0.082, green: 0.016, blue: 0.18)
  }
  public var card_foreground: Color {
    return Color(red: 0.969, green: 0.961, blue: 0.988)
  }
  public var popover: Color {
    return Color(red: 0.082, green: 0.016, blue: 0.18)
  }
  public var popover_foreground: Color {
    return Color(red: 0.969, green: 0.961, blue: 0.988)
  }
  public var primary: Color {
    return Color(red: 0.427, green: 0.153, blue: 0.851)
  }
  public var primary_foreground: Color {
    return Color(red: 0.969, green: 0.961, blue: 0.988)
  }
  public var secondary: Color {
    return Color(red: 0.157, green: 0.098, blue: 0.267)
  }
  public var secondary_foreground: Color {
    return Color(red: 0.969, green: 0.961, blue: 0.988)
  }
  public var muted: Color {
    return Color(red: 0.157, green: 0.098, blue: 0.267)
  }
  public var muted_foreground: Color {
    return Color(red: 0.631, green: 0.619, blue: 0.682)
  }
  public var accent: Color {
    return Color(red: 0.157, green: 0.098, blue: 0.267)
  }
  public var accent_foreground: Color {
    return Color(red: 0.969, green: 0.961, blue: 0.988)
  }
  public var destructive: Color {
    return Color(red: 0.498, green: 0.114, blue: 0.114)
  }
  public var destructive_foreground: Color {
    return Color(red: 0.969, green: 0.961, blue: 0.988)
  }
  public var border: Color {
    return Color(red: 0.157, green: 0.098, blue: 0.267)
  }
  public var input: Color {
    return Color(red: 0.157, green: 0.098, blue: 0.267)
  }
  public var ring: Color {
    return Color(red: 0.427, green: 0.153, blue: 0.851)
  }
  public var general_link_color: Color {
    return Color(red: 0.118, green: 0.565, blue: 1)
  }
}

public struct FlusterDark: ThemeProtocol {
  public typealias ItemType = WebViewTheme
  public var id: WebViewTheme = .fluster
  public init() {}
  public var background: Color {
    return Color(red: 0.043, green: 0.082, blue: 0.153)
  }
  public var foreground: Color {
    return Color(red: 0.969, green: 0.98, blue: 0.988)
  }
  public var card: Color {
    return Color(red: 0.043, green: 0.082, blue: 0.153)
  }
  public var card_foreground: Color {
    return Color(red: 0.969, green: 0.98, blue: 0.988)
  }
  public var popover: Color {
    return Color(red: 0.043, green: 0.082, blue: 0.153)
  }
  public var popover_foreground: Color {
    return Color(red: 0.969, green: 0.98, blue: 0.988)
  }
  public var primary: Color {
    return Color(red: 0.231, green: 0.506, blue: 0.965)
  }
  public var primary_foreground: Color {
    return Color(red: 0.969, green: 0.98, blue: 0.988)
  }
  public var secondary: Color {
    return Color(red: 0.153, green: 0.22, blue: 0.286)
  }
  public var secondary_foreground: Color {
    return Color(red: 0.969, green: 0.98, blue: 0.988)
  }
  public var muted: Color {
    return Color(red: 0.153, green: 0.22, blue: 0.286)
  }
  public var muted_foreground: Color {
    return Color(red: 0.616, green: 0.655, blue: 0.698)
  }
  public var accent: Color {
    return Color(red: 0.153, green: 0.22, blue: 0.286)
  }
  public var accent_foreground: Color {
    return Color(red: 0.969, green: 0.98, blue: 0.988)
  }
  public var destructive: Color { return Color(red: 1, green: 0, blue: 0) }
  public var destructive_foreground: Color {
    return Color(red: 0.969, green: 0.98, blue: 0.988)
  }
  public var border: Color {
    return Color(red: 0.153, green: 0.22, blue: 0.286)
  }
  public var input: Color {
    return Color(red: 0.153, green: 0.22, blue: 0.286)
  }
  public var ring: Color {
    return Color(red: 0.125, green: 0.424, blue: 0.922)
  }
  public var general_link_color: Color {
    return Color(red: 0.118, green: 0.565, blue: 1)
  }
}

public struct StoneDark: ThemeProtocol {
  public typealias ItemType = WebViewTheme
  public var id: WebViewTheme = .stone
  public init() {}
  public var background: Color {
    return Color(red: 0.102, green: 0.086, blue: 0.059)
  }
  public var foreground: Color {
    return Color(red: 0.98, green: 0.98, blue: 0.976)
  }
  public var card: Color {
    return Color(red: 0.102, green: 0.086, blue: 0.059)
  }
  public var card_foreground: Color {
    return Color(red: 0.98, green: 0.98, blue: 0.976)
  }
  public var popover: Color {
    return Color(red: 0.102, green: 0.086, blue: 0.059)
  }
  public var popover_foreground: Color {
    return Color(red: 0.98, green: 0.98, blue: 0.976)
  }
  public var primary: Color {
    return Color(red: 0.98, green: 0.98, blue: 0.976)
  }
  public var primary_foreground: Color {
    return Color(red: 0.106, green: 0.098, blue: 0.09)
  }
  public var secondary: Color {
    return Color(red: 0.145, green: 0.122, blue: 0.098)
  }
  public var secondary_foreground: Color {
    return Color(red: 0.98, green: 0.98, blue: 0.976)
  }
  public var muted: Color {
    return Color(red: 0.145, green: 0.122, blue: 0.098)
  }
  public var muted_foreground: Color {
    return Color(red: 0.647, green: 0.62, blue: 0.573)
  }
  public var accent: Color {
    return Color(red: 0.145, green: 0.122, blue: 0.098)
  }
  public var accent_foreground: Color {
    return Color(red: 0.98, green: 0.98, blue: 0.976)
  }
  public var destructive: Color {
    return Color(red: 0.498, green: 0.114, blue: 0.114)
  }
  public var destructive_foreground: Color {
    return Color(red: 0.98, green: 0.98, blue: 0.976)
  }
  public var border: Color {
    return Color(red: 0.145, green: 0.122, blue: 0.098)
  }
  public var input: Color {
    return Color(red: 0.145, green: 0.122, blue: 0.098)
  }
  public var ring: Color {
    return Color(red: 0.851, green: 0.835, blue: 0.816)
  }
  public var general_link_color: Color {
    return Color(red: 0.118, green: 0.565, blue: 1)
  }
}

public struct SlateDark: ThemeProtocol {
  public typealias ItemType = WebViewTheme
  public var id: WebViewTheme = .slate
  public init() {}
  public var background: Color {
    return Color(red: 0.043, green: 0.082, blue: 0.153)
  }
  public var foreground: Color {
    return Color(red: 0.969, green: 0.98, blue: 0.988)
  }
  public var card: Color {
    return Color(red: 0.043, green: 0.082, blue: 0.153)
  }
  public var card_foreground: Color {
    return Color(red: 0.969, green: 0.98, blue: 0.988)
  }
  public var popover: Color {
    return Color(red: 0.043, green: 0.082, blue: 0.153)
  }
  public var popover_foreground: Color {
    return Color(red: 0.969, green: 0.98, blue: 0.988)
  }
  public var primary: Color {
    return Color(red: 0.969, green: 0.98, blue: 0.988)
  }
  public var primary_foreground: Color {
    return Color(red: 0.063, green: 0.09, blue: 0.165)
  }
  public var secondary: Color {
    return Color(red: 0.153, green: 0.22, blue: 0.286)
  }
  public var secondary_foreground: Color {
    return Color(red: 0.969, green: 0.98, blue: 0.988)
  }
  public var muted: Color {
    return Color(red: 0.153, green: 0.22, blue: 0.286)
  }
  public var muted_foreground: Color {
    return Color(red: 0.616, green: 0.655, blue: 0.698)
  }
  public var accent: Color {
    return Color(red: 0.153, green: 0.22, blue: 0.286)
  }
  public var accent_foreground: Color {
    return Color(red: 0.969, green: 0.98, blue: 0.988)
  }
  public var destructive: Color {
    return Color(red: 0.498, green: 0.114, blue: 0.114)
  }
  public var destructive_foreground: Color {
    return Color(red: 0.969, green: 0.98, blue: 0.988)
  }
  public var border: Color {
    return Color(red: 0.153, green: 0.22, blue: 0.286)
  }
  public var input: Color {
    return Color(red: 0.153, green: 0.22, blue: 0.286)
  }
  public var ring: Color {
    return Color(red: 0.816, green: 0.855, blue: 0.898)
  }
  public var general_link_color: Color {
    return Color(red: 0.118, green: 0.565, blue: 1)
  }
}

public struct RoseDark: ThemeProtocol {
  public typealias ItemType = WebViewTheme
  public var id: WebViewTheme = .rose
  public init() {}
  public var background: Color {
    return Color(red: 0.102, green: 0.086, blue: 0.059)
  }
  public var foreground: Color {
    return Color(red: 0.949, green: 0.949, blue: 0.949)
  }
  public var card: Color {
    return Color(red: 0.106, green: 0.098, blue: 0.09)
  }
  public var card_foreground: Color {
    return Color(red: 0.949, green: 0.949, blue: 0.949)
  }
  public var popover: Color {
    return Color(red: 0.09, green: 0.09, blue: 0.09)
  }
  public var popover_foreground: Color {
    return Color(red: 0.949, green: 0.949, blue: 0.949)
  }
  public var primary: Color {
    return Color(red: 0.882, green: 0.114, blue: 0.278)
  }
  public var primary_foreground: Color {
    return Color(red: 0.996, green: 0.953, blue: 0.961)
  }
  public var secondary: Color {
    return Color(red: 0.153, green: 0.153, blue: 0.165)
  }
  public var secondary_foreground: Color {
    return Color(red: 0.98, green: 0.98, blue: 0.98)
  }
  public var muted: Color {
    return Color(red: 0.149, green: 0.149, blue: 0.149)
  }
  public var muted_foreground: Color {
    return Color(red: 0.631, green: 0.631, blue: 0.667)
  }
  public var accent: Color {
    return Color(red: 0.145, green: 0.122, blue: 0.098)
  }
  public var accent_foreground: Color {
    return Color(red: 0.98, green: 0.98, blue: 0.98)
  }
  public var destructive: Color {
    return Color(red: 0.498, green: 0.114, blue: 0.114)
  }
  public var destructive_foreground: Color {
    return Color(red: 0.996, green: 0.953, blue: 0.961)
  }
  public var border: Color {
    return Color(red: 0.153, green: 0.153, blue: 0.165)
  }
  public var input: Color {
    return Color(red: 0.153, green: 0.153, blue: 0.165)
  }
  public var ring: Color {
    return Color(red: 0.882, green: 0.114, blue: 0.278)
  }
  public var general_link_color: Color {
    return Color(red: 0.118, green: 0.565, blue: 1)
  }
}

public struct RedDark: ThemeProtocol {
  public typealias ItemType = WebViewTheme
  public var id: WebViewTheme = .red
  public init() {}
  public var background: Color {
    return Color(red: 0.039, green: 0.039, blue: 0.039)
  }
  public var foreground: Color {
    return Color(red: 0.98, green: 0.98, blue: 0.98)
  }
  public var card: Color {
    return Color(red: 0.039, green: 0.039, blue: 0.039)
  }
  public var card_foreground: Color {
    return Color(red: 0.98, green: 0.98, blue: 0.98)
  }
  public var popover: Color {
    return Color(red: 0.039, green: 0.039, blue: 0.039)
  }
  public var popover_foreground: Color {
    return Color(red: 0.98, green: 0.98, blue: 0.98)
  }
  public var primary: Color {
    return Color(red: 0.863, green: 0.149, blue: 0.153)
  }
  public var primary_foreground: Color {
    return Color(red: 0.996, green: 0.953, blue: 0.961)
  }
  public var secondary: Color {
    return Color(red: 0.149, green: 0.149, blue: 0.149)
  }
  public var secondary_foreground: Color {
    return Color(red: 0.98, green: 0.98, blue: 0.98)
  }
  public var muted: Color {
    return Color(red: 0.149, green: 0.149, blue: 0.149)
  }
  public var muted_foreground: Color {
    return Color(red: 0.64, green: 0.64, blue: 0.64)
  }
  public var accent: Color {
    return Color(red: 0.149, green: 0.149, blue: 0.149)
  }
  public var accent_foreground: Color {
    return Color(red: 0.98, green: 0.98, blue: 0.98)
  }
  public var destructive: Color {
    return Color(red: 0.498, green: 0.114, blue: 0.114)
  }
  public var destructive_foreground: Color {
    return Color(red: 0.98, green: 0.98, blue: 0.98)
  }
  public var border: Color {
    return Color(red: 0.149, green: 0.149, blue: 0.149)
  }
  public var input: Color {
    return Color(red: 0.149, green: 0.149, blue: 0.149)
  }
  public var ring: Color {
    return Color(red: 0.863, green: 0.149, blue: 0.153)
  }
  public var general_link_color: Color {
    return Color(red: 0.118, green: 0.565, blue: 1)
  }
}

public struct OrangeDark: ThemeProtocol {
  public typealias ItemType = WebViewTheme
  public var id: WebViewTheme = .orange
  public init() {}
  public var background: Color {
    return Color(red: 0.102, green: 0.086, blue: 0.059)
  }
  public var foreground: Color {
    return Color(red: 0.98, green: 0.98, blue: 0.976)
  }
  public var card: Color {
    return Color(red: 0.102, green: 0.086, blue: 0.059)
  }
  public var card_foreground: Color {
    return Color(red: 0.98, green: 0.98, blue: 0.976)
  }
  public var popover: Color {
    return Color(red: 0.102, green: 0.086, blue: 0.059)
  }
  public var popover_foreground: Color {
    return Color(red: 0.98, green: 0.98, blue: 0.976)
  }
  public var primary: Color {
    return Color(red: 0.918, green: 0.345, blue: 0.043)
  }
  public var primary_foreground: Color {
    return Color(red: 0.98, green: 0.98, blue: 0.976)
  }
  public var secondary: Color {
    return Color(red: 0.145, green: 0.122, blue: 0.098)
  }
  public var secondary_foreground: Color {
    return Color(red: 0.98, green: 0.98, blue: 0.976)
  }
  public var muted: Color {
    return Color(red: 0.145, green: 0.122, blue: 0.098)
  }
  public var muted_foreground: Color {
    return Color(red: 0.647, green: 0.62, blue: 0.573)
  }
  public var accent: Color {
    return Color(red: 0.145, green: 0.122, blue: 0.098)
  }
  public var accent_foreground: Color {
    return Color(red: 0.98, green: 0.98, blue: 0.976)
  }
  public var destructive: Color {
    return Color(red: 0.863, green: 0.149, blue: 0.153)
  }
  public var destructive_foreground: Color {
    return Color(red: 0.98, green: 0.98, blue: 0.976)
  }
  public var border: Color {
    return Color(red: 0.145, green: 0.122, blue: 0.098)
  }
  public var input: Color {
    return Color(red: 0.145, green: 0.122, blue: 0.098)
  }
  public var ring: Color {
    return Color(red: 0.918, green: 0.345, blue: 0.043)
  }
  public var general_link_color: Color {
    return Color(red: 0.118, green: 0.565, blue: 1)
  }
}

public struct NeutralDark: ThemeProtocol {
  public typealias ItemType = WebViewTheme
  public var id: WebViewTheme = .neutral
  public init() {}
  public var background: Color {
    return Color(red: 0.039, green: 0.039, blue: 0.039)
  }
  public var foreground: Color {
    return Color(red: 0.98, green: 0.98, blue: 0.98)
  }
  public var card: Color {
    return Color(red: 0.039, green: 0.039, blue: 0.039)
  }
  public var card_foreground: Color {
    return Color(red: 0.98, green: 0.98, blue: 0.98)
  }
  public var popover: Color {
    return Color(red: 0.039, green: 0.039, blue: 0.039)
  }
  public var popover_foreground: Color {
    return Color(red: 0.98, green: 0.98, blue: 0.98)
  }
  public var primary: Color {
    return Color(red: 0.98, green: 0.98, blue: 0.98)
  }
  public var primary_foreground: Color {
    return Color(red: 0.09, green: 0.09, blue: 0.09)
  }
  public var secondary: Color {
    return Color(red: 0.149, green: 0.149, blue: 0.149)
  }
  public var secondary_foreground: Color {
    return Color(red: 0.98, green: 0.98, blue: 0.98)
  }
  public var muted: Color {
    return Color(red: 0.149, green: 0.149, blue: 0.149)
  }
  public var muted_foreground: Color {
    return Color(red: 0.64, green: 0.64, blue: 0.64)
  }
  public var accent: Color {
    return Color(red: 0.149, green: 0.149, blue: 0.149)
  }
  public var accent_foreground: Color {
    return Color(red: 0.98, green: 0.98, blue: 0.98)
  }
  public var destructive: Color {
    return Color(red: 0.498, green: 0.114, blue: 0.114)
  }
  public var destructive_foreground: Color {
    return Color(red: 0.98, green: 0.98, blue: 0.98)
  }
  public var border: Color {
    return Color(red: 0.149, green: 0.149, blue: 0.149)
  }
  public var input: Color {
    return Color(red: 0.149, green: 0.149, blue: 0.149)
  }
  public var ring: Color {
    return Color(red: 0.831, green: 0.831, blue: 0.831)
  }
  public var general_link_color: Color {
    return Color(red: 0.118, green: 0.565, blue: 1)
  }
}

public struct GreenDark: ThemeProtocol {
  public typealias ItemType = WebViewTheme
  public var id: WebViewTheme = .green
  public init() {}
  public var background: Color {
    return Color(red: 0.102, green: 0.086, blue: 0.059)
  }
  public var foreground: Color {
    return Color(red: 0.949, green: 0.949, blue: 0.949)
  }
  public var card: Color {
    return Color(red: 0.106, green: 0.098, blue: 0.09)
  }
  public var card_foreground: Color {
    return Color(red: 0.949, green: 0.949, blue: 0.949)
  }
  public var popover: Color {
    return Color(red: 0.09, green: 0.09, blue: 0.09)
  }
  public var popover_foreground: Color {
    return Color(red: 0.949, green: 0.949, blue: 0.949)
  }
  public var primary: Color {
    return Color(red: 0.13, green: 0.773, blue: 0.369)
  }
  public var primary_foreground: Color {
    return Color(red: 0.102, green: 0.196, blue: 0.102)
  }
  public var secondary: Color {
    return Color(red: 0.153, green: 0.153, blue: 0.165)
  }
  public var secondary_foreground: Color {
    return Color(red: 0.98, green: 0.98, blue: 0.98)
  }
  public var muted: Color {
    return Color(red: 0.149, green: 0.149, blue: 0.149)
  }
  public var muted_foreground: Color {
    return Color(red: 0.631, green: 0.631, blue: 0.667)
  }
  public var accent: Color {
    return Color(red: 0.145, green: 0.122, blue: 0.098)
  }
  public var accent_foreground: Color {
    return Color(red: 0.98, green: 0.98, blue: 0.98)
  }
  public var destructive: Color {
    return Color(red: 0.498, green: 0.114, blue: 0.114)
  }
  public var destructive_foreground: Color {
    return Color(red: 0.996, green: 0.953, blue: 0.961)
  }
  public var border: Color {
    return Color(red: 0.153, green: 0.153, blue: 0.165)
  }
  public var input: Color {
    return Color(red: 0.153, green: 0.153, blue: 0.165)
  }
  public var ring: Color {
    return Color(red: 0.094, green: 0.53, blue: 0.231)
  }
  public var general_link_color: Color {
    return Color(red: 0.118, green: 0.565, blue: 1)
  }
}

public struct GrayDark: ThemeProtocol {
  public typealias ItemType = WebViewTheme
  public var id: WebViewTheme = .gray
  public init() {}
  public var background: Color {
    return Color(red: 0.082, green: 0.016, blue: 0.18)
  }
  public var foreground: Color {
    return Color(red: 0.969, green: 0.961, blue: 0.988)
  }
  public var card: Color {
    return Color(red: 0.082, green: 0.016, blue: 0.18)
  }
  public var card_foreground: Color {
    return Color(red: 0.969, green: 0.961, blue: 0.988)
  }
  public var popover: Color {
    return Color(red: 0.082, green: 0.016, blue: 0.18)
  }
  public var popover_foreground: Color {
    return Color(red: 0.969, green: 0.961, blue: 0.988)
  }
  public var primary: Color {
    return Color(red: 0.969, green: 0.961, blue: 0.988)
  }
  public var primary_foreground: Color {
    return Color(red: 0.118, green: 0.059, blue: 0.271)
  }
  public var secondary: Color {
    return Color(red: 0.157, green: 0.098, blue: 0.267)
  }
  public var secondary_foreground: Color {
    return Color(red: 0.969, green: 0.961, blue: 0.988)
  }
  public var muted: Color {
    return Color(red: 0.157, green: 0.098, blue: 0.267)
  }
  public var muted_foreground: Color {
    return Color(red: 0.631, green: 0.619, blue: 0.682)
  }
  public var accent: Color {
    return Color(red: 0.157, green: 0.098, blue: 0.267)
  }
  public var accent_foreground: Color {
    return Color(red: 0.969, green: 0.961, blue: 0.988)
  }
  public var destructive: Color {
    return Color(red: 0.498, green: 0.114, blue: 0.114)
  }
  public var destructive_foreground: Color {
    return Color(red: 0.969, green: 0.961, blue: 0.988)
  }
  public var border: Color {
    return Color(red: 0.157, green: 0.098, blue: 0.267)
  }
  public var input: Color {
    return Color(red: 0.157, green: 0.098, blue: 0.267)
  }
  public var ring: Color {
    return Color(red: 0.839, green: 0.847, blue: 0.867)
  }
  public var general_link_color: Color {
    return Color(red: 0.118, green: 0.565, blue: 1)
  }
}

public struct BlueDark: ThemeProtocol {
  public typealias ItemType = WebViewTheme
  public var id: WebViewTheme = .blue
  public init() {}
  public var background: Color {
    return Color(red: 0.043, green: 0.082, blue: 0.153)
  }
  public var foreground: Color {
    return Color(red: 0.969, green: 0.98, blue: 0.988)
  }
  public var card: Color {
    return Color(red: 0.043, green: 0.082, blue: 0.153)
  }
  public var card_foreground: Color {
    return Color(red: 0.969, green: 0.98, blue: 0.988)
  }
  public var popover: Color {
    return Color(red: 0.043, green: 0.082, blue: 0.153)
  }
  public var popover_foreground: Color {
    return Color(red: 0.969, green: 0.98, blue: 0.988)
  }
  public var primary: Color {
    return Color(red: 0.231, green: 0.506, blue: 0.965)
  }
  public var primary_foreground: Color {
    return Color(red: 0.063, green: 0.09, blue: 0.165)
  }
  public var secondary: Color {
    return Color(red: 0.153, green: 0.22, blue: 0.286)
  }
  public var secondary_foreground: Color {
    return Color(red: 0.969, green: 0.98, blue: 0.988)
  }
  public var muted: Color {
    return Color(red: 0.153, green: 0.22, blue: 0.286)
  }
  public var muted_foreground: Color {
    return Color(red: 0.616, green: 0.655, blue: 0.698)
  }
  public var accent: Color {
    return Color(red: 0.153, green: 0.22, blue: 0.286)
  }
  public var accent_foreground: Color {
    return Color(red: 0.969, green: 0.98, blue: 0.988)
  }
  public var destructive: Color {
    return Color(red: 0.498, green: 0.114, blue: 0.114)
  }
  public var destructive_foreground: Color {
    return Color(red: 0.969, green: 0.98, blue: 0.988)
  }
  public var border: Color {
    return Color(red: 0.153, green: 0.22, blue: 0.286)
  }
  public var input: Color {
    return Color(red: 0.153, green: 0.22, blue: 0.286)
  }
  public var ring: Color {
    return Color(red: 0.125, green: 0.424, blue: 0.922)
  }
  public var general_link_color: Color {
    return Color(red: 0.118, green: 0.565, blue: 1)
  }
}

// -- Light Themes --

public struct ZincLight: ThemeProtocol {
  public typealias ItemType = WebViewTheme
  public var id: WebViewTheme = .zinc
  public init() {}
  public var background: Color { return Color(red: 1, green: 1, blue: 1) }
  public var foreground: Color {
    return Color(red: 0.051, green: 0.051, blue: 0.059)
  }
  public var card: Color { return Color(red: 1, green: 1, blue: 1) }
  public var card_foreground: Color {
    return Color(red: 0.051, green: 0.051, blue: 0.059)
  }
  public var popover: Color { return Color(red: 1, green: 1, blue: 1) }
  public var popover_foreground: Color {
    return Color(red: 0.051, green: 0.051, blue: 0.059)
  }
  public var primary: Color {
    return Color(red: 0.094, green: 0.094, blue: 0.098)
  }
  public var primary_foreground: Color {
    return Color(red: 0.98, green: 0.98, blue: 0.98)
  }
  public var secondary: Color {
    return Color(red: 0.957, green: 0.957, blue: 0.961)
  }
  public var secondary_foreground: Color {
    return Color(red: 0.094, green: 0.094, blue: 0.098)
  }
  public var muted: Color {
    return Color(red: 0.957, green: 0.957, blue: 0.961)
  }
  public var muted_foreground: Color {
    return Color(red: 0.443, green: 0.443, blue: 0.482)
  }
  public var accent: Color {
    return Color(red: 0.957, green: 0.957, blue: 0.961)
  }
  public var accent_foreground: Color {
    return Color(red: 0.094, green: 0.094, blue: 0.098)
  }
  public var destructive: Color {
    return Color(red: 0.937, green: 0.267, blue: 0.267)
  }
  public var destructive_foreground: Color {
    return Color(red: 0.98, green: 0.98, blue: 0.98)
  }
  public var border: Color {
    return Color(red: 0.894, green: 0.894, blue: 0.906)
  }
  public var input: Color {
    return Color(red: 0.894, green: 0.894, blue: 0.906)
  }
  public var ring: Color {
    return Color(red: 0.094, green: 0.094, blue: 0.098)
  }
  public var brand: Color {
    return Color(red: 0.043, green: 0.647, blue: 0.914)
  }
  public var general_link_color: Color {
    return Color(red: 0.145, green: 0.39, blue: 0.922)
  }
}

public struct YellowLight: ThemeProtocol {
  public typealias ItemType = WebViewTheme
  public var id: WebViewTheme = .yellow
  public init() {}
  public var background: Color { return Color(red: 1, green: 1, blue: 1) }
  public var foreground: Color {
    return Color(red: 0.102, green: 0.086, blue: 0.059)
  }
  public var card: Color { return Color(red: 1, green: 1, blue: 1) }
  public var card_foreground: Color {
    return Color(red: 0.102, green: 0.086, blue: 0.059)
  }
  public var popover: Color { return Color(red: 1, green: 1, blue: 1) }
  public var popover_foreground: Color {
    return Color(red: 0.102, green: 0.086, blue: 0.059)
  }
  public var primary: Color {
    return Color(red: 0.98, green: 0.8, blue: 0.078)
  }
  public var primary_foreground: Color {
    return Color(red: 0.2, green: 0.137, blue: 0.02)
  }
  public var secondary: Color {
    return Color(red: 0.957, green: 0.957, blue: 0.961)
  }
  public var secondary_foreground: Color {
    return Color(red: 0.106, green: 0.098, blue: 0.09)
  }
  public var muted: Color {
    return Color(red: 0.957, green: 0.957, blue: 0.961)
  }
  public var muted_foreground: Color {
    return Color(red: 0.455, green: 0.431, blue: 0.384)
  }
  public var accent: Color {
    return Color(red: 0.957, green: 0.957, blue: 0.961)
  }
  public var accent_foreground: Color {
    return Color(red: 0.106, green: 0.098, blue: 0.09)
  }
  public var destructive: Color {
    return Color(red: 0.937, green: 0.267, blue: 0.267)
  }
  public var destructive_foreground: Color {
    return Color(red: 0.98, green: 0.98, blue: 0.976)
  }
  public var border: Color {
    return Color(red: 0.902, green: 0.898, blue: 0.89)
  }
  public var input: Color {
    return Color(red: 0.902, green: 0.898, blue: 0.89)
  }
  public var ring: Color {
    return Color(red: 0.102, green: 0.086, blue: 0.059)
  }
  public var brand: Color {
    return Color(red: 0.043, green: 0.647, blue: 0.914)
  }
  public var general_link_color: Color {
    return Color(red: 0.145, green: 0.39, blue: 0.922)
  }
}

public struct VioletLight: ThemeProtocol {
  public typealias ItemType = WebViewTheme
  public var id: WebViewTheme = .violet
  public init() {}
  public var background: Color { return Color(red: 1, green: 1, blue: 1) }
  public var foreground: Color {
    return Color(red: 0.082, green: 0.016, blue: 0.18)
  }
  public var card: Color { return Color(red: 1, green: 1, blue: 1) }
  public var card_foreground: Color {
    return Color(red: 0.082, green: 0.016, blue: 0.18)
  }
  public var popover: Color { return Color(red: 1, green: 1, blue: 1) }
  public var popover_foreground: Color {
    return Color(red: 0.082, green: 0.016, blue: 0.18)
  }
  public var primary: Color {
    return Color(red: 0.482, green: 0.224, blue: 0.929)
  }
  public var primary_foreground: Color {
    return Color(red: 0.969, green: 0.961, blue: 0.988)
  }
  public var secondary: Color {
    return Color(red: 0.945, green: 0.949, blue: 0.973)
  }
  public var secondary_foreground: Color {
    return Color(red: 0.118, green: 0.059, blue: 0.271)
  }
  public var muted: Color {
    return Color(red: 0.945, green: 0.949, blue: 0.973)
  }
  public var muted_foreground: Color {
    return Color(red: 0.471, green: 0.463, blue: 0.529)
  }
  public var accent: Color {
    return Color(red: 0.945, green: 0.949, blue: 0.973)
  }
  public var accent_foreground: Color {
    return Color(red: 0.118, green: 0.059, blue: 0.271)
  }
  public var destructive: Color {
    return Color(red: 0.937, green: 0.267, blue: 0.267)
  }
  public var destructive_foreground: Color {
    return Color(red: 0.969, green: 0.961, blue: 0.988)
  }
  public var border: Color {
    return Color(red: 0.898, green: 0.902, blue: 0.922)
  }
  public var input: Color {
    return Color(red: 0.898, green: 0.902, blue: 0.922)
  }
  public var ring: Color {
    return Color(red: 0.482, green: 0.224, blue: 0.929)
  }
  public var brand: Color {
    return Color(red: 0.043, green: 0.647, blue: 0.914)
  }
  public var general_link_color: Color {
    return Color(red: 0.145, green: 0.39, blue: 0.922)
  }
}

public struct FlusterLight: ThemeProtocol {
  public typealias ItemType = WebViewTheme
  public var id: WebViewTheme = .fluster
  public init() {}
  public var background: Color { return Color(red: 1, green: 1, blue: 1) }
  public var foreground: Color {
    return Color(red: 0.043, green: 0.082, blue: 0.153)
  }
  public var card: Color { return Color(red: 1, green: 1, blue: 1) }
  public var card_foreground: Color {
    return Color(red: 0.043, green: 0.082, blue: 0.153)
  }
  public var popover: Color { return Color(red: 1, green: 1, blue: 1) }
  public var popover_foreground: Color {
    return Color(red: 0.043, green: 0.082, blue: 0.153)
  }
  public var primary: Color {
    return Color(red: 0.043, green: 0.647, blue: 0.914)
  }
  public var primary_foreground: Color {
    return Color(red: 0.969, green: 0.98, blue: 0.988)
  }
  public var secondary: Color {
    return Color(red: 0.941, green: 0.961, blue: 0.976)
  }
  public var secondary_foreground: Color {
    return Color(red: 0.063, green: 0.09, blue: 0.165)
  }
  public var muted: Color {
    return Color(red: 0.941, green: 0.961, blue: 0.976)
  }
  public var muted_foreground: Color {
    return Color(red: 0.471, green: 0.53, blue: 0.608)
  }
  public var accent: Color {
    return Color(red: 0.941, green: 0.961, blue: 0.976)
  }
  public var accent_foreground: Color {
    return Color(red: 0.063, green: 0.09, blue: 0.165)
  }
  public var destructive: Color {
    return Color(red: 0.937, green: 0.267, blue: 0.267)
  }
  public var destructive_foreground: Color {
    return Color(red: 0.969, green: 0.98, blue: 0.988)
  }
  public var border: Color {
    return Color(red: 0.902, green: 0.922, blue: 0.941)
  }
  public var input: Color {
    return Color(red: 0.902, green: 0.922, blue: 0.941)
  }
  public var ring: Color {
    return Color(red: 0.22, green: 0.749, blue: 0.506)
  }
  public var brand: Color {
    return Color(red: 0.043, green: 0.647, blue: 0.914)
  }
  public var general_link_color: Color {
    return Color(red: 0.145, green: 0.39, blue: 0.922)
  }
}

public struct StoneLight: ThemeProtocol {
  public typealias ItemType = WebViewTheme
  public var id: WebViewTheme = .stone
  public init() {}
  public var background: Color { return Color(red: 1, green: 1, blue: 1) }
  public var foreground: Color {
    return Color(red: 0.102, green: 0.086, blue: 0.059)
  }
  public var card: Color { return Color(red: 1, green: 1, blue: 1) }
  public var card_foreground: Color {
    return Color(red: 0.102, green: 0.086, blue: 0.059)
  }
  public var popover: Color { return Color(red: 1, green: 1, blue: 1) }
  public var popover_foreground: Color {
    return Color(red: 0.102, green: 0.086, blue: 0.059)
  }
  public var primary: Color {
    return Color(red: 0.106, green: 0.098, blue: 0.09)
  }
  public var primary_foreground: Color {
    return Color(red: 0.98, green: 0.98, blue: 0.976)
  }
  public var secondary: Color {
    return Color(red: 0.957, green: 0.957, blue: 0.961)
  }
  public var secondary_foreground: Color {
    return Color(red: 0.106, green: 0.098, blue: 0.09)
  }
  public var muted: Color {
    return Color(red: 0.957, green: 0.957, blue: 0.961)
  }
  public var muted_foreground: Color {
    return Color(red: 0.455, green: 0.431, blue: 0.384)
  }
  public var accent: Color {
    return Color(red: 0.957, green: 0.957, blue: 0.961)
  }
  public var accent_foreground: Color {
    return Color(red: 0.106, green: 0.098, blue: 0.09)
  }
  public var destructive: Color {
    return Color(red: 0.937, green: 0.267, blue: 0.267)
  }
  public var destructive_foreground: Color {
    return Color(red: 0.98, green: 0.98, blue: 0.976)
  }
  public var border: Color {
    return Color(red: 0.902, green: 0.898, blue: 0.89)
  }
  public var input: Color {
    return Color(red: 0.902, green: 0.898, blue: 0.89)
  }
  public var ring: Color {
    return Color(red: 0.102, green: 0.086, blue: 0.059)
  }
  public var brand: Color {
    return Color(red: 0.043, green: 0.647, blue: 0.914)
  }
  public var general_link_color: Color {
    return Color(red: 0.145, green: 0.39, blue: 0.922)
  }
}

public struct SlateLight: ThemeProtocol {
  public typealias ItemType = WebViewTheme
  public var id: WebViewTheme = .slate
  public init() {}
  public var background: Color { return Color(red: 1, green: 1, blue: 1) }
  public var foreground: Color {
    return Color(red: 0.043, green: 0.082, blue: 0.153)
  }
  public var card: Color { return Color(red: 1, green: 1, blue: 1) }
  public var card_foreground: Color {
    return Color(red: 0.043, green: 0.082, blue: 0.153)
  }
  public var popover: Color { return Color(red: 1, green: 1, blue: 1) }
  public var popover_foreground: Color {
    return Color(red: 0.043, green: 0.082, blue: 0.153)
  }
  public var primary: Color {
    return Color(red: 0.063, green: 0.09, blue: 0.165)
  }
  public var primary_foreground: Color {
    return Color(red: 0.969, green: 0.98, blue: 0.988)
  }
  public var secondary: Color {
    return Color(red: 0.941, green: 0.961, blue: 0.976)
  }
  public var secondary_foreground: Color {
    return Color(red: 0.063, green: 0.09, blue: 0.165)
  }
  public var muted: Color {
    return Color(red: 0.941, green: 0.961, blue: 0.976)
  }
  public var muted_foreground: Color {
    return Color(red: 0.471, green: 0.53, blue: 0.608)
  }
  public var accent: Color {
    return Color(red: 0.941, green: 0.961, blue: 0.976)
  }
  public var accent_foreground: Color {
    return Color(red: 0.063, green: 0.09, blue: 0.165)
  }
  public var destructive: Color {
    return Color(red: 0.937, green: 0.267, blue: 0.267)
  }
  public var destructive_foreground: Color {
    return Color(red: 0.969, green: 0.98, blue: 0.988)
  }
  public var border: Color {
    return Color(red: 0.902, green: 0.922, blue: 0.941)
  }
  public var input: Color {
    return Color(red: 0.902, green: 0.922, blue: 0.941)
  }
  public var ring: Color {
    return Color(red: 0.043, green: 0.082, blue: 0.153)
  }
  public var brand: Color {
    return Color(red: 0.043, green: 0.647, blue: 0.914)
  }
  public var general_link_color: Color {
    return Color(red: 0.145, green: 0.39, blue: 0.922)
  }
}

struct RoseLight: ThemeProtocol {
  public typealias ItemType = WebViewTheme
  public var id: WebViewTheme = .rose
  public init() {}
  public var background: Color { return Color(red: 1, green: 1, blue: 1) }
  public var foreground: Color {
    return Color(red: 0.051, green: 0.051, blue: 0.059)
  }
  public var card: Color { return Color(red: 1, green: 1, blue: 1) }
  public var card_foreground: Color {
    return Color(red: 0.051, green: 0.051, blue: 0.059)
  }
  public var popover: Color { return Color(red: 1, green: 1, blue: 1) }
  public var popover_foreground: Color {
    return Color(red: 0.051, green: 0.051, blue: 0.059)
  }
  public var primary: Color {
    return Color(red: 0.882, green: 0.114, blue: 0.278)
  }
  public var primary_foreground: Color {
    return Color(red: 0.996, green: 0.953, blue: 0.961)
  }
  public var secondary: Color {
    return Color(red: 0.957, green: 0.957, blue: 0.961)
  }
  public var secondary_foreground: Color {
    return Color(red: 0.094, green: 0.094, blue: 0.098)
  }
  public var muted: Color {
    return Color(red: 0.957, green: 0.957, blue: 0.961)
  }
  public var muted_foreground: Color {
    return Color(red: 0.443, green: 0.443, blue: 0.482)
  }
  public var accent: Color {
    return Color(red: 0.957, green: 0.957, blue: 0.961)
  }
  public var accent_foreground: Color {
    return Color(red: 0.094, green: 0.094, blue: 0.098)
  }
  public var destructive: Color {
    return Color(red: 0.937, green: 0.267, blue: 0.267)
  }
  public var destructive_foreground: Color {
    return Color(red: 0.98, green: 0.98, blue: 0.98)
  }
  public var border: Color {
    return Color(red: 0.894, green: 0.894, blue: 0.906)
  }
  public var input: Color {
    return Color(red: 0.894, green: 0.894, blue: 0.906)
  }
  public var ring: Color {
    return Color(red: 0.882, green: 0.114, blue: 0.278)
  }
  public var brand: Color {
    return Color(red: 0.043, green: 0.647, blue: 0.914)
  }
  public var general_link_color: Color {
    return Color(red: 0.145, green: 0.39, blue: 0.922)
  }
}

struct RedLight: ThemeProtocol {
  public typealias ItemType = WebViewTheme
  public var id: WebViewTheme = .red
  public init() {}
  public var background: Color { return Color(red: 1, green: 1, blue: 1) }
  public var foreground: Color {
    return Color(red: 0.039, green: 0.039, blue: 0.039)
  }
  public var card: Color { return Color(red: 1, green: 1, blue: 1) }
  public var card_foreground: Color {
    return Color(red: 0.039, green: 0.039, blue: 0.039)
  }
  public var popover: Color { return Color(red: 1, green: 1, blue: 1) }
  public var popover_foreground: Color {
    return Color(red: 0.039, green: 0.039, blue: 0.039)
  }
  public var primary: Color {
    return Color(red: 0.863, green: 0.149, blue: 0.153)
  }
  public var primary_foreground: Color {
    return Color(red: 0.996, green: 0.953, blue: 0.961)
  }
  public var secondary: Color {
    return Color(red: 0.961, green: 0.961, blue: 0.961)
  }
  public var secondary_foreground: Color {
    return Color(red: 0.09, green: 0.09, blue: 0.09)
  }
  public var muted: Color {
    return Color(red: 0.961, green: 0.961, blue: 0.961)
  }
  public var muted_foreground: Color {
    return Color(red: 0.451, green: 0.451, blue: 0.451)
  }
  public var accent: Color {
    return Color(red: 0.961, green: 0.961, blue: 0.961)
  }
  public var accent_foreground: Color {
    return Color(red: 0.09, green: 0.09, blue: 0.09)
  }
  public var destructive: Color {
    return Color(red: 0.937, green: 0.267, blue: 0.267)
  }
  public var destructive_foreground: Color {
    return Color(red: 0.98, green: 0.98, blue: 0.98)
  }
  public var border: Color {
    return Color(red: 0.898, green: 0.898, blue: 0.898)
  }
  public var input: Color {
    return Color(red: 0.898, green: 0.898, blue: 0.898)
  }
  public var ring: Color {
    return Color(red: 0.863, green: 0.149, blue: 0.153)
  }
  public var brand: Color {
    return Color(red: 0.043, green: 0.647, blue: 0.914)
  }
  public var general_link_color: Color {
    return Color(red: 0.145, green: 0.39, blue: 0.922)
  }
}

struct OrangeLight: ThemeProtocol {
  public typealias ItemType = WebViewTheme
  public var id: WebViewTheme = .orange
  public init() {}
  public var background: Color { return Color(red: 1, green: 1, blue: 1) }
  public var foreground: Color {
    return Color(red: 0.102, green: 0.086, blue: 0.059)
  }
  public var card: Color { return Color(red: 1, green: 1, blue: 1) }
  public var card_foreground: Color {
    return Color(red: 0.102, green: 0.086, blue: 0.059)
  }
  public var popover: Color { return Color(red: 1, green: 1, blue: 1) }
  public var popover_foreground: Color {
    return Color(red: 0.102, green: 0.086, blue: 0.059)
  }
  public var primary: Color {
    return Color(red: 0.973, green: 0.451, blue: 0.082)
  }
  public var primary_foreground: Color {
    return Color(red: 0.98, green: 0.98, blue: 0.976)
  }
  public var secondary: Color {
    return Color(red: 0.957, green: 0.957, blue: 0.961)
  }
  public var secondary_foreground: Color {
    return Color(red: 0.106, green: 0.098, blue: 0.09)
  }
  public var muted: Color {
    return Color(red: 0.957, green: 0.957, blue: 0.961)
  }
  public var muted_foreground: Color {
    return Color(red: 0.455, green: 0.431, blue: 0.384)
  }
  public var accent: Color {
    return Color(red: 0.957, green: 0.957, blue: 0.961)
  }
  public var accent_foreground: Color {
    return Color(red: 0.106, green: 0.098, blue: 0.09)
  }
  public var destructive: Color {
    return Color(red: 0.937, green: 0.267, blue: 0.267)
  }
  public var destructive_foreground: Color {
    return Color(red: 0.98, green: 0.98, blue: 0.976)
  }
  public var border: Color {
    return Color(red: 0.902, green: 0.898, blue: 0.89)
  }
  public var input: Color {
    return Color(red: 0.902, green: 0.898, blue: 0.89)
  }
  public var ring: Color {
    return Color(red: 0.973, green: 0.451, blue: 0.082)
  }
  public var brand: Color {
    return Color(red: 0.043, green: 0.647, blue: 0.914)
  }
  public var general_link_color: Color {
    return Color(red: 0.145, green: 0.39, blue: 0.922)
  }
}

struct NeutralLight: ThemeProtocol {
  public typealias ItemType = WebViewTheme
  public var id: WebViewTheme = .neutral
  public init() {}
  public var background: Color { return Color(red: 1, green: 1, blue: 1) }
  public var foreground: Color {
    return Color(red: 0.039, green: 0.039, blue: 0.039)
  }
  public var card: Color { return Color(red: 1, green: 1, blue: 1) }
  public var card_foreground: Color {
    return Color(red: 0.039, green: 0.039, blue: 0.039)
  }
  public var popover: Color { return Color(red: 1, green: 1, blue: 1) }
  public var popover_foreground: Color {
    return Color(red: 0.039, green: 0.039, blue: 0.039)
  }
  public var primary: Color {
    return Color(red: 0.09, green: 0.09, blue: 0.09)
  }
  public var primary_foreground: Color {
    return Color(red: 0.98, green: 0.98, blue: 0.98)
  }
  public var secondary: Color {
    return Color(red: 0.961, green: 0.961, blue: 0.961)
  }
  public var secondary_foreground: Color {
    return Color(red: 0.09, green: 0.09, blue: 0.09)
  }
  public var muted: Color {
    return Color(red: 0.961, green: 0.961, blue: 0.961)
  }
  public var muted_foreground: Color {
    return Color(red: 0.451, green: 0.451, blue: 0.451)
  }
  public var accent: Color {
    return Color(red: 0.961, green: 0.961, blue: 0.961)
  }
  public var accent_foreground: Color {
    return Color(red: 0.09, green: 0.09, blue: 0.09)
  }
  public var destructive: Color {
    return Color(red: 0.937, green: 0.267, blue: 0.267)
  }
  public var destructive_foreground: Color {
    return Color(red: 0.98, green: 0.98, blue: 0.98)
  }
  public var border: Color {
    return Color(red: 0.898, green: 0.898, blue: 0.898)
  }
  public var input: Color {
    return Color(red: 0.898, green: 0.898, blue: 0.898)
  }
  public var ring: Color {
    return Color(red: 0.039, green: 0.039, blue: 0.039)
  }
  public var brand: Color {
    return Color(red: 0.043, green: 0.647, blue: 0.914)
  }
  public var general_link_color: Color {
    return Color(red: 0.145, green: 0.39, blue: 0.922)
  }
}

struct GreenLight: ThemeProtocol {
  public typealias ItemType = WebViewTheme
  public var id: WebViewTheme = .green
  public init() {}
  public var background: Color { return Color(red: 1, green: 1, blue: 1) }
  public var foreground: Color {
    return Color(red: 0.051, green: 0.051, blue: 0.059)
  }
  public var card: Color { return Color(red: 1, green: 1, blue: 1) }
  public var card_foreground: Color {
    return Color(red: 0.051, green: 0.051, blue: 0.059)
  }
  public var popover: Color { return Color(red: 1, green: 1, blue: 1) }
  public var popover_foreground: Color {
    return Color(red: 0.051, green: 0.051, blue: 0.059)
  }
  public var primary: Color {
    return Color(red: 0.086, green: 0.64, blue: 0.286)
  }
  public var primary_foreground: Color {
    return Color(red: 0.996, green: 0.953, blue: 0.961)
  }
  public var secondary: Color {
    return Color(red: 0.957, green: 0.957, blue: 0.961)
  }
  public var secondary_foreground: Color {
    return Color(red: 0.094, green: 0.094, blue: 0.098)
  }
  public var muted: Color {
    return Color(red: 0.957, green: 0.957, blue: 0.961)
  }
  public var muted_foreground: Color {
    return Color(red: 0.443, green: 0.443, blue: 0.482)
  }
  public var accent: Color {
    return Color(red: 0.957, green: 0.957, blue: 0.961)
  }
  public var accent_foreground: Color {
    return Color(red: 0.094, green: 0.094, blue: 0.098)
  }
  public var destructive: Color {
    return Color(red: 0.937, green: 0.267, blue: 0.267)
  }
  public var destructive_foreground: Color {
    return Color(red: 0.98, green: 0.98, blue: 0.98)
  }
  public var border: Color {
    return Color(red: 0.894, green: 0.894, blue: 0.906)
  }
  public var input: Color {
    return Color(red: 0.894, green: 0.894, blue: 0.906)
  }
  public var ring: Color {
    return Color(red: 0.086, green: 0.64, blue: 0.286)
  }
  public var brand: Color {
    return Color(red: 0.043, green: 0.647, blue: 0.914)
  }
  public var general_link_color: Color {
    return Color(red: 0.145, green: 0.39, blue: 0.922)
  }
}

struct GrayLight: ThemeProtocol {
  public typealias ItemType = WebViewTheme
  public var id: WebViewTheme = .gray
  public init() {}
  public var background: Color { return Color(red: 1, green: 1, blue: 1) }
  public var foreground: Color {
    return Color(red: 0.082, green: 0.016, blue: 0.18)
  }
  public var card: Color { return Color(red: 1, green: 1, blue: 1) }
  public var card_foreground: Color {
    return Color(red: 0.082, green: 0.016, blue: 0.18)
  }
  public var popover: Color { return Color(red: 1, green: 1, blue: 1) }
  public var popover_foreground: Color {
    return Color(red: 0.082, green: 0.016, blue: 0.18)
  }
  public var primary: Color {
    return Color(red: 0.118, green: 0.059, blue: 0.271)
  }
  public var primary_foreground: Color {
    return Color(red: 0.969, green: 0.961, blue: 0.988)
  }
  public var secondary: Color {
    return Color(red: 0.945, green: 0.949, blue: 0.973)
  }
  public var secondary_foreground: Color {
    return Color(red: 0.118, green: 0.059, blue: 0.271)
  }
  public var muted: Color {
    return Color(red: 0.945, green: 0.949, blue: 0.973)
  }
  public var muted_foreground: Color {
    return Color(red: 0.471, green: 0.463, blue: 0.529)
  }
  public var accent: Color {
    return Color(red: 0.945, green: 0.949, blue: 0.973)
  }
  public var accent_foreground: Color {
    return Color(red: 0.118, green: 0.059, blue: 0.271)
  }
  public var destructive: Color {
    return Color(red: 0.937, green: 0.267, blue: 0.267)
  }
  public var destructive_foreground: Color {
    return Color(red: 0.969, green: 0.961, blue: 0.988)
  }
  public var border: Color {
    return Color(red: 0.898, green: 0.902, blue: 0.922)
  }
  public var input: Color {
    return Color(red: 0.898, green: 0.902, blue: 0.922)
  }
  public var ring: Color {
    return Color(red: 0.082, green: 0.016, blue: 0.18)
  }
  public var brand: Color {
    return Color(red: 0.043, green: 0.647, blue: 0.914)
  }
  public var general_link_color: Color {
    return Color(red: 0.145, green: 0.39, blue: 0.922)
  }
}

struct BlueLight: ThemeProtocol {
  typealias ItemType = WebViewTheme
  public var id: WebViewTheme = .blue
  public init() {}
  public var background: Color { return Color(red: 1, green: 1, blue: 1) }
  public var foreground: Color {
    return Color(red: 0.043, green: 0.082, blue: 0.153)
  }
  public var card: Color { return Color(red: 1, green: 1, blue: 1) }
  public var card_foreground: Color {
    return Color(red: 0.043, green: 0.082, blue: 0.153)
  }
  public var popover: Color { return Color(red: 1, green: 1, blue: 1) }
  public var popover_foreground: Color {
    return Color(red: 0.043, green: 0.082, blue: 0.153)
  }
  public var primary: Color {
    return Color(red: 0.141, green: 0.388, blue: 0.922)
  }
  public var primary_foreground: Color {
    return Color(red: 0.969, green: 0.98, blue: 0.988)
  }
  public var secondary: Color {
    return Color(red: 0.941, green: 0.961, blue: 0.976)
  }
  public var secondary_foreground: Color {
    return Color(red: 0.063, green: 0.09, blue: 0.165)
  }
  public var muted: Color {
    return Color(red: 0.941, green: 0.961, blue: 0.976)
  }
  public var muted_foreground: Color {
    return Color(red: 0.471, green: 0.53, blue: 0.608)
  }
  public var accent: Color {
    return Color(red: 0.941, green: 0.961, blue: 0.976)
  }
  public var accent_foreground: Color {
    return Color(red: 0.063, green: 0.09, blue: 0.165)
  }
  public var destructive: Color {
    return Color(red: 0.937, green: 0.267, blue: 0.267)
  }
  public var destructive_foreground: Color {
    return Color(red: 0.969, green: 0.98, blue: 0.988)
  }
  public var border: Color {
    return Color(red: 0.902, green: 0.922, blue: 0.941)
  }
  public var input: Color {
    return Color(red: 0.902, green: 0.922, blue: 0.941)
  }
  public var ring: Color {
    return Color(red: 0.141, green: 0.388, blue: 0.922)
  }
  public var brand: Color {
    return Color(red: 0.043, green: 0.647, blue: 0.914)
  }
  public var general_link_color: Color {
    return Color(red: 0.145, green: 0.39, blue: 0.922)
  }
}

public func getTheme(themeName: WebViewTheme, darkMode: Bool) -> any ThemeProtocol {
  switch themeName {
    case .zinc:
      if darkMode {
        return ZincDark()
      } else {
        return ZincLight()
      }
    case .yellow:
      if darkMode {
        return YellowDark()
      } else {
        return YellowLight()
      }
    case .violet:
      if darkMode {
        return VioletDark()
      } else {
        return VioletLight()
      }
    case .fluster:
      if darkMode {
        return FlusterDark()
      } else {
        return FlusterLight()
      }
    case .stone:
      if darkMode {
        return StoneDark()
      } else {
        return StoneLight()
      }
    case .slate:
      if darkMode {
        return SlateDark()
      } else {
        return SlateLight()
      }
    case .rose:
      if darkMode {
        return RoseDark()
      } else {
        return RoseLight()
      }
    case .red:
      if darkMode {
        return RedDark()
      } else {
        return RedLight()
      }
    case .orange:
      if darkMode {
        return OrangeDark()
      } else {
        return OrangeLight()
      }
    case .neutral:
      if darkMode {
        return NeutralDark()
      } else {
        return NeutralLight()
      }
    case .green:
      if darkMode {
        return GreenDark()
      } else {
        return GreenLight()
      }
    case .gray:
      if darkMode {
        return GrayDark()
      } else {
        return GrayLight()
      }
    case .blue:
      if darkMode {
        return BlueDark()
      } else {
        return BlueLight()
      }
  }
}
