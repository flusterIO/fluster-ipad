import SwiftUI

enum WebViewTheme: String, Codable, CaseIterable {
    case fluster, zinc, yellow, violet, stone, slate, rose, red, orange, green, gray, blue, neutral
}

// -- Dark Themes --


struct ZincDark: ThemeProtocol {
    typealias ItemType = WebViewTheme
    var id: WebViewTheme = .zinc
    var background: Color { return Color(red: 0.051, green: 0.051, blue: 0.059) }
    var foreground: Color { return Color(red: 0.98, green: 0.98, blue: 0.98) }
    var card: Color { return Color(red: 0.051, green: 0.051, blue: 0.059) }
    var card_foreground: Color { return Color(red: 0.98, green: 0.98, blue: 0.98) }
    var popover: Color { return Color(red: 0.051, green: 0.051, blue: 0.059) }
    var popover_foreground: Color { return Color(red: 0.98, green: 0.98, blue: 0.98) }
    var primary: Color { return Color(red: 0.98, green: 0.98, blue: 0.98) }
    var primary_foreground: Color { return Color(red: 0.094, green: 0.094, blue: 0.098) }
    var secondary: Color { return Color(red: 0.153, green: 0.153, blue: 0.165) }
    var secondary_foreground: Color { return Color(red: 0.98, green: 0.98, blue: 0.98) }
    var muted: Color { return Color(red: 0.153, green: 0.153, blue: 0.165) }
    var muted_foreground: Color { return Color(red: 0.631, green: 0.631, blue: 0.667) }
    var accent: Color { return Color(red: 0.153, green: 0.153, blue: 0.165) }
    var accent_foreground: Color { return Color(red: 0.98, green: 0.98, blue: 0.98) }
    var destructive: Color { return Color(red: 0.498, green: 0.114, blue: 0.114) }
    var destructive_foreground: Color { return Color(red: 0.98, green: 0.98, blue: 0.98) }
    var border: Color { return Color(red: 0.153, green: 0.153, blue: 0.165) }
    var input: Color { return Color(red: 0.153, green: 0.153, blue: 0.165) }
    var ring: Color { return Color(red: 0.831, green: 0.831, blue: 0.847) }
    var general_link_color: Color { return Color(red: 0.118, green: 0.565, blue: 1) }

} 

struct YellowDark: ThemeProtocol {
    typealias ItemType = WebViewTheme
    var id: WebViewTheme = .yellow
    var background: Color { return Color(red: 0.102, green: 0.086, blue: 0.059) }
    var foreground: Color { return Color(red: 0.98, green: 0.98, blue: 0.976) }
    var card: Color { return Color(red: 0.102, green: 0.086, blue: 0.059) }
    var card_foreground: Color { return Color(red: 0.98, green: 0.98, blue: 0.976) }
    var popover: Color { return Color(red: 0.102, green: 0.086, blue: 0.059) }
    var popover_foreground: Color { return Color(red: 0.98, green: 0.98, blue: 0.976) }
    var primary: Color { return Color(red: 0.98, green: 0.8, blue: 0.078) }
    var primary_foreground: Color { return Color(red: 0.2, green: 0.137, blue: 0.02) }
    var secondary: Color { return Color(red: 0.145, green: 0.122, blue: 0.098) }
    var secondary_foreground: Color { return Color(red: 0.98, green: 0.98, blue: 0.976) }
    var muted: Color { return Color(red: 0.145, green: 0.122, blue: 0.098) }
    var muted_foreground: Color { return Color(red: 0.647, green: 0.62, blue: 0.573) }
    var accent: Color { return Color(red: 0.145, green: 0.122, blue: 0.098) }
    var accent_foreground: Color { return Color(red: 0.98, green: 0.98, blue: 0.976) }
    var destructive: Color { return Color(red: 0.498, green: 0.114, blue: 0.114) }
    var destructive_foreground: Color { return Color(red: 0.98, green: 0.98, blue: 0.976) }
    var border: Color { return Color(red: 0.145, green: 0.122, blue: 0.098) }
    var input: Color { return Color(red: 0.145, green: 0.122, blue: 0.098) }
    var ring: Color { return Color(red: 0.898, green: 0.678, blue: 0.169) }
    var general_link_color: Color { return Color(red: 0.118, green: 0.565, blue: 1) }

} 

struct VioletDark: ThemeProtocol {
    typealias ItemType = WebViewTheme
    var id: WebViewTheme = .violet
    var background: Color { return Color(red: 0.082, green: 0.016, blue: 0.18) }
    var foreground: Color { return Color(red: 0.969, green: 0.961, blue: 0.988) }
    var card: Color { return Color(red: 0.082, green: 0.016, blue: 0.18) }
    var card_foreground: Color { return Color(red: 0.969, green: 0.961, blue: 0.988) }
    var popover: Color { return Color(red: 0.082, green: 0.016, blue: 0.18) }
    var popover_foreground: Color { return Color(red: 0.969, green: 0.961, blue: 0.988) }
    var primary: Color { return Color(red: 0.427, green: 0.153, blue: 0.851) }
    var primary_foreground: Color { return Color(red: 0.969, green: 0.961, blue: 0.988) }
    var secondary: Color { return Color(red: 0.157, green: 0.098, blue: 0.267) }
    var secondary_foreground: Color { return Color(red: 0.969, green: 0.961, blue: 0.988) }
    var muted: Color { return Color(red: 0.157, green: 0.098, blue: 0.267) }
    var muted_foreground: Color { return Color(red: 0.631, green: 0.619, blue: 0.682) }
    var accent: Color { return Color(red: 0.157, green: 0.098, blue: 0.267) }
    var accent_foreground: Color { return Color(red: 0.969, green: 0.961, blue: 0.988) }
    var destructive: Color { return Color(red: 0.498, green: 0.114, blue: 0.114) }
    var destructive_foreground: Color { return Color(red: 0.969, green: 0.961, blue: 0.988) }
    var border: Color { return Color(red: 0.157, green: 0.098, blue: 0.267) }
    var input: Color { return Color(red: 0.157, green: 0.098, blue: 0.267) }
    var ring: Color { return Color(red: 0.427, green: 0.153, blue: 0.851) }
    var general_link_color: Color { return Color(red: 0.118, green: 0.565, blue: 1) }

} 

struct FlusterDark: ThemeProtocol {
    typealias ItemType = WebViewTheme
    var id: WebViewTheme = .fluster
    var background: Color { return Color(red: 0.043, green: 0.082, blue: 0.153) }
    var foreground: Color { return Color(red: 0.969, green: 0.98, blue: 0.988) }
    var card: Color { return Color(red: 0.043, green: 0.082, blue: 0.153) }
    var card_foreground: Color { return Color(red: 0.969, green: 0.98, blue: 0.988) }
    var popover: Color { return Color(red: 0.043, green: 0.082, blue: 0.153) }
    var popover_foreground: Color { return Color(red: 0.969, green: 0.98, blue: 0.988) }
    var primary: Color { return Color(red: 0.231, green: 0.506, blue: 0.965) }
    var primary_foreground: Color { return Color(red: 0.969, green: 0.98, blue: 0.988) }
    var secondary: Color { return Color(red: 0.153, green: 0.22, blue: 0.286) }
    var secondary_foreground: Color { return Color(red: 0.969, green: 0.98, blue: 0.988) }
    var muted: Color { return Color(red: 0.153, green: 0.22, blue: 0.286) }
    var muted_foreground: Color { return Color(red: 0.616, green: 0.655, blue: 0.698) }
    var accent: Color { return Color(red: 0.153, green: 0.22, blue: 0.286) }
    var accent_foreground: Color { return Color(red: 0.969, green: 0.98, blue: 0.988) }
    var destructive: Color { return Color(red: 1, green: 0, blue: 0) }
    var destructive_foreground: Color { return Color(red: 0.969, green: 0.98, blue: 0.988) }
    var border: Color { return Color(red: 0.153, green: 0.22, blue: 0.286) }
    var input: Color { return Color(red: 0.153, green: 0.22, blue: 0.286) }
    var ring: Color { return Color(red: 0.125, green: 0.424, blue: 0.922) }
    var general_link_color: Color { return Color(red: 0.118, green: 0.565, blue: 1) }

} 

struct StoneDark: ThemeProtocol {
    typealias ItemType = WebViewTheme
    var id: WebViewTheme = .stone
    var background: Color { return Color(red: 0.102, green: 0.086, blue: 0.059) }
    var foreground: Color { return Color(red: 0.98, green: 0.98, blue: 0.976) }
    var card: Color { return Color(red: 0.102, green: 0.086, blue: 0.059) }
    var card_foreground: Color { return Color(red: 0.98, green: 0.98, blue: 0.976) }
    var popover: Color { return Color(red: 0.102, green: 0.086, blue: 0.059) }
    var popover_foreground: Color { return Color(red: 0.98, green: 0.98, blue: 0.976) }
    var primary: Color { return Color(red: 0.98, green: 0.98, blue: 0.976) }
    var primary_foreground: Color { return Color(red: 0.106, green: 0.098, blue: 0.09) }
    var secondary: Color { return Color(red: 0.145, green: 0.122, blue: 0.098) }
    var secondary_foreground: Color { return Color(red: 0.98, green: 0.98, blue: 0.976) }
    var muted: Color { return Color(red: 0.145, green: 0.122, blue: 0.098) }
    var muted_foreground: Color { return Color(red: 0.647, green: 0.62, blue: 0.573) }
    var accent: Color { return Color(red: 0.145, green: 0.122, blue: 0.098) }
    var accent_foreground: Color { return Color(red: 0.98, green: 0.98, blue: 0.976) }
    var destructive: Color { return Color(red: 0.498, green: 0.114, blue: 0.114) }
    var destructive_foreground: Color { return Color(red: 0.98, green: 0.98, blue: 0.976) }
    var border: Color { return Color(red: 0.145, green: 0.122, blue: 0.098) }
    var input: Color { return Color(red: 0.145, green: 0.122, blue: 0.098) }
    var ring: Color { return Color(red: 0.851, green: 0.835, blue: 0.816) }
    var general_link_color: Color { return Color(red: 0.118, green: 0.565, blue: 1) }

} 

struct SlateDark: ThemeProtocol {
    typealias ItemType = WebViewTheme
    var id: WebViewTheme = .slate
    var background: Color { return Color(red: 0.043, green: 0.082, blue: 0.153) }
    var foreground: Color { return Color(red: 0.969, green: 0.98, blue: 0.988) }
    var card: Color { return Color(red: 0.043, green: 0.082, blue: 0.153) }
    var card_foreground: Color { return Color(red: 0.969, green: 0.98, blue: 0.988) }
    var popover: Color { return Color(red: 0.043, green: 0.082, blue: 0.153) }
    var popover_foreground: Color { return Color(red: 0.969, green: 0.98, blue: 0.988) }
    var primary: Color { return Color(red: 0.969, green: 0.98, blue: 0.988) }
    var primary_foreground: Color { return Color(red: 0.063, green: 0.09, blue: 0.165) }
    var secondary: Color { return Color(red: 0.153, green: 0.22, blue: 0.286) }
    var secondary_foreground: Color { return Color(red: 0.969, green: 0.98, blue: 0.988) }
    var muted: Color { return Color(red: 0.153, green: 0.22, blue: 0.286) }
    var muted_foreground: Color { return Color(red: 0.616, green: 0.655, blue: 0.698) }
    var accent: Color { return Color(red: 0.153, green: 0.22, blue: 0.286) }
    var accent_foreground: Color { return Color(red: 0.969, green: 0.98, blue: 0.988) }
    var destructive: Color { return Color(red: 0.498, green: 0.114, blue: 0.114) }
    var destructive_foreground: Color { return Color(red: 0.969, green: 0.98, blue: 0.988) }
    var border: Color { return Color(red: 0.153, green: 0.22, blue: 0.286) }
    var input: Color { return Color(red: 0.153, green: 0.22, blue: 0.286) }
    var ring: Color { return Color(red: 0.816, green: 0.855, blue: 0.898) }
    var general_link_color: Color { return Color(red: 0.118, green: 0.565, blue: 1) }

} 

struct RoseDark: ThemeProtocol {
    typealias ItemType = WebViewTheme
    var id: WebViewTheme = .rose
    var background: Color { return Color(red: 0.102, green: 0.086, blue: 0.059) }
    var foreground: Color { return Color(red: 0.949, green: 0.949, blue: 0.949) }
    var card: Color { return Color(red: 0.106, green: 0.098, blue: 0.09) }
    var card_foreground: Color { return Color(red: 0.949, green: 0.949, blue: 0.949) }
    var popover: Color { return Color(red: 0.09, green: 0.09, blue: 0.09) }
    var popover_foreground: Color { return Color(red: 0.949, green: 0.949, blue: 0.949) }
    var primary: Color { return Color(red: 0.882, green: 0.114, blue: 0.278) }
    var primary_foreground: Color { return Color(red: 0.996, green: 0.953, blue: 0.961) }
    var secondary: Color { return Color(red: 0.153, green: 0.153, blue: 0.165) }
    var secondary_foreground: Color { return Color(red: 0.98, green: 0.98, blue: 0.98) }
    var muted: Color { return Color(red: 0.149, green: 0.149, blue: 0.149) }
    var muted_foreground: Color { return Color(red: 0.631, green: 0.631, blue: 0.667) }
    var accent: Color { return Color(red: 0.145, green: 0.122, blue: 0.098) }
    var accent_foreground: Color { return Color(red: 0.98, green: 0.98, blue: 0.98) }
    var destructive: Color { return Color(red: 0.498, green: 0.114, blue: 0.114) }
    var destructive_foreground: Color { return Color(red: 0.996, green: 0.953, blue: 0.961) }
    var border: Color { return Color(red: 0.153, green: 0.153, blue: 0.165) }
    var input: Color { return Color(red: 0.153, green: 0.153, blue: 0.165) }
    var ring: Color { return Color(red: 0.882, green: 0.114, blue: 0.278) }
    var general_link_color: Color { return Color(red: 0.118, green: 0.565, blue: 1) }

} 

struct RedDark: ThemeProtocol {
    typealias ItemType = WebViewTheme
    var id: WebViewTheme = .red
    var background: Color { return Color(red: 0.039, green: 0.039, blue: 0.039) }
    var foreground: Color { return Color(red: 0.98, green: 0.98, blue: 0.98) }
    var card: Color { return Color(red: 0.039, green: 0.039, blue: 0.039) }
    var card_foreground: Color { return Color(red: 0.98, green: 0.98, blue: 0.98) }
    var popover: Color { return Color(red: 0.039, green: 0.039, blue: 0.039) }
    var popover_foreground: Color { return Color(red: 0.98, green: 0.98, blue: 0.98) }
    var primary: Color { return Color(red: 0.863, green: 0.149, blue: 0.153) }
    var primary_foreground: Color { return Color(red: 0.996, green: 0.953, blue: 0.961) }
    var secondary: Color { return Color(red: 0.149, green: 0.149, blue: 0.149) }
    var secondary_foreground: Color { return Color(red: 0.98, green: 0.98, blue: 0.98) }
    var muted: Color { return Color(red: 0.149, green: 0.149, blue: 0.149) }
    var muted_foreground: Color { return Color(red: 0.64, green: 0.64, blue: 0.64) }
    var accent: Color { return Color(red: 0.149, green: 0.149, blue: 0.149) }
    var accent_foreground: Color { return Color(red: 0.98, green: 0.98, blue: 0.98) }
    var destructive: Color { return Color(red: 0.498, green: 0.114, blue: 0.114) }
    var destructive_foreground: Color { return Color(red: 0.98, green: 0.98, blue: 0.98) }
    var border: Color { return Color(red: 0.149, green: 0.149, blue: 0.149) }
    var input: Color { return Color(red: 0.149, green: 0.149, blue: 0.149) }
    var ring: Color { return Color(red: 0.863, green: 0.149, blue: 0.153) }
    var general_link_color: Color { return Color(red: 0.118, green: 0.565, blue: 1) }

} 

struct OrangeDark: ThemeProtocol {
    typealias ItemType = WebViewTheme
    var id: WebViewTheme = .orange
    var background: Color { return Color(red: 0.102, green: 0.086, blue: 0.059) }
    var foreground: Color { return Color(red: 0.98, green: 0.98, blue: 0.976) }
    var card: Color { return Color(red: 0.102, green: 0.086, blue: 0.059) }
    var card_foreground: Color { return Color(red: 0.98, green: 0.98, blue: 0.976) }
    var popover: Color { return Color(red: 0.102, green: 0.086, blue: 0.059) }
    var popover_foreground: Color { return Color(red: 0.98, green: 0.98, blue: 0.976) }
    var primary: Color { return Color(red: 0.918, green: 0.345, blue: 0.043) }
    var primary_foreground: Color { return Color(red: 0.98, green: 0.98, blue: 0.976) }
    var secondary: Color { return Color(red: 0.145, green: 0.122, blue: 0.098) }
    var secondary_foreground: Color { return Color(red: 0.98, green: 0.98, blue: 0.976) }
    var muted: Color { return Color(red: 0.145, green: 0.122, blue: 0.098) }
    var muted_foreground: Color { return Color(red: 0.647, green: 0.62, blue: 0.573) }
    var accent: Color { return Color(red: 0.145, green: 0.122, blue: 0.098) }
    var accent_foreground: Color { return Color(red: 0.98, green: 0.98, blue: 0.976) }
    var destructive: Color { return Color(red: 0.863, green: 0.149, blue: 0.153) }
    var destructive_foreground: Color { return Color(red: 0.98, green: 0.98, blue: 0.976) }
    var border: Color { return Color(red: 0.145, green: 0.122, blue: 0.098) }
    var input: Color { return Color(red: 0.145, green: 0.122, blue: 0.098) }
    var ring: Color { return Color(red: 0.918, green: 0.345, blue: 0.043) }
    var general_link_color: Color { return Color(red: 0.118, green: 0.565, blue: 1) }

} 

struct NeutralDark: ThemeProtocol {
    typealias ItemType = WebViewTheme
    var id: WebViewTheme = .neutral
    var background: Color { return Color(red: 0.039, green: 0.039, blue: 0.039) }
    var foreground: Color { return Color(red: 0.98, green: 0.98, blue: 0.98) }
    var card: Color { return Color(red: 0.039, green: 0.039, blue: 0.039) }
    var card_foreground: Color { return Color(red: 0.98, green: 0.98, blue: 0.98) }
    var popover: Color { return Color(red: 0.039, green: 0.039, blue: 0.039) }
    var popover_foreground: Color { return Color(red: 0.98, green: 0.98, blue: 0.98) }
    var primary: Color { return Color(red: 0.98, green: 0.98, blue: 0.98) }
    var primary_foreground: Color { return Color(red: 0.09, green: 0.09, blue: 0.09) }
    var secondary: Color { return Color(red: 0.149, green: 0.149, blue: 0.149) }
    var secondary_foreground: Color { return Color(red: 0.98, green: 0.98, blue: 0.98) }
    var muted: Color { return Color(red: 0.149, green: 0.149, blue: 0.149) }
    var muted_foreground: Color { return Color(red: 0.64, green: 0.64, blue: 0.64) }
    var accent: Color { return Color(red: 0.149, green: 0.149, blue: 0.149) }
    var accent_foreground: Color { return Color(red: 0.98, green: 0.98, blue: 0.98) }
    var destructive: Color { return Color(red: 0.498, green: 0.114, blue: 0.114) }
    var destructive_foreground: Color { return Color(red: 0.98, green: 0.98, blue: 0.98) }
    var border: Color { return Color(red: 0.149, green: 0.149, blue: 0.149) }
    var input: Color { return Color(red: 0.149, green: 0.149, blue: 0.149) }
    var ring: Color { return Color(red: 0.831, green: 0.831, blue: 0.831) }
    var general_link_color: Color { return Color(red: 0.118, green: 0.565, blue: 1) }

} 

struct GreenDark: ThemeProtocol {
    typealias ItemType = WebViewTheme
    var id: WebViewTheme = .green
    var background: Color { return Color(red: 0.102, green: 0.086, blue: 0.059) }
    var foreground: Color { return Color(red: 0.949, green: 0.949, blue: 0.949) }
    var card: Color { return Color(red: 0.106, green: 0.098, blue: 0.09) }
    var card_foreground: Color { return Color(red: 0.949, green: 0.949, blue: 0.949) }
    var popover: Color { return Color(red: 0.09, green: 0.09, blue: 0.09) }
    var popover_foreground: Color { return Color(red: 0.949, green: 0.949, blue: 0.949) }
    var primary: Color { return Color(red: 0.13, green: 0.773, blue: 0.369) }
    var primary_foreground: Color { return Color(red: 0.102, green: 0.196, blue: 0.102) }
    var secondary: Color { return Color(red: 0.153, green: 0.153, blue: 0.165) }
    var secondary_foreground: Color { return Color(red: 0.98, green: 0.98, blue: 0.98) }
    var muted: Color { return Color(red: 0.149, green: 0.149, blue: 0.149) }
    var muted_foreground: Color { return Color(red: 0.631, green: 0.631, blue: 0.667) }
    var accent: Color { return Color(red: 0.145, green: 0.122, blue: 0.098) }
    var accent_foreground: Color { return Color(red: 0.98, green: 0.98, blue: 0.98) }
    var destructive: Color { return Color(red: 0.498, green: 0.114, blue: 0.114) }
    var destructive_foreground: Color { return Color(red: 0.996, green: 0.953, blue: 0.961) }
    var border: Color { return Color(red: 0.153, green: 0.153, blue: 0.165) }
    var input: Color { return Color(red: 0.153, green: 0.153, blue: 0.165) }
    var ring: Color { return Color(red: 0.094, green: 0.53, blue: 0.231) }
    var general_link_color: Color { return Color(red: 0.118, green: 0.565, blue: 1) }

} 

struct GrayDark: ThemeProtocol {
    typealias ItemType = WebViewTheme
    var id: WebViewTheme = .gray
    var background: Color { return Color(red: 0.082, green: 0.016, blue: 0.18) }
    var foreground: Color { return Color(red: 0.969, green: 0.961, blue: 0.988) }
    var card: Color { return Color(red: 0.082, green: 0.016, blue: 0.18) }
    var card_foreground: Color { return Color(red: 0.969, green: 0.961, blue: 0.988) }
    var popover: Color { return Color(red: 0.082, green: 0.016, blue: 0.18) }
    var popover_foreground: Color { return Color(red: 0.969, green: 0.961, blue: 0.988) }
    var primary: Color { return Color(red: 0.969, green: 0.961, blue: 0.988) }
    var primary_foreground: Color { return Color(red: 0.118, green: 0.059, blue: 0.271) }
    var secondary: Color { return Color(red: 0.157, green: 0.098, blue: 0.267) }
    var secondary_foreground: Color { return Color(red: 0.969, green: 0.961, blue: 0.988) }
    var muted: Color { return Color(red: 0.157, green: 0.098, blue: 0.267) }
    var muted_foreground: Color { return Color(red: 0.631, green: 0.619, blue: 0.682) }
    var accent: Color { return Color(red: 0.157, green: 0.098, blue: 0.267) }
    var accent_foreground: Color { return Color(red: 0.969, green: 0.961, blue: 0.988) }
    var destructive: Color { return Color(red: 0.498, green: 0.114, blue: 0.114) }
    var destructive_foreground: Color { return Color(red: 0.969, green: 0.961, blue: 0.988) }
    var border: Color { return Color(red: 0.157, green: 0.098, blue: 0.267) }
    var input: Color { return Color(red: 0.157, green: 0.098, blue: 0.267) }
    var ring: Color { return Color(red: 0.839, green: 0.847, blue: 0.867) }
    var general_link_color: Color { return Color(red: 0.118, green: 0.565, blue: 1) }

} 

struct BlueDark: ThemeProtocol {
    typealias ItemType = WebViewTheme
    var id: WebViewTheme = .blue
    var background: Color { return Color(red: 0.043, green: 0.082, blue: 0.153) }
    var foreground: Color { return Color(red: 0.969, green: 0.98, blue: 0.988) }
    var card: Color { return Color(red: 0.043, green: 0.082, blue: 0.153) }
    var card_foreground: Color { return Color(red: 0.969, green: 0.98, blue: 0.988) }
    var popover: Color { return Color(red: 0.043, green: 0.082, blue: 0.153) }
    var popover_foreground: Color { return Color(red: 0.969, green: 0.98, blue: 0.988) }
    var primary: Color { return Color(red: 0.231, green: 0.506, blue: 0.965) }
    var primary_foreground: Color { return Color(red: 0.063, green: 0.09, blue: 0.165) }
    var secondary: Color { return Color(red: 0.153, green: 0.22, blue: 0.286) }
    var secondary_foreground: Color { return Color(red: 0.969, green: 0.98, blue: 0.988) }
    var muted: Color { return Color(red: 0.153, green: 0.22, blue: 0.286) }
    var muted_foreground: Color { return Color(red: 0.616, green: 0.655, blue: 0.698) }
    var accent: Color { return Color(red: 0.153, green: 0.22, blue: 0.286) }
    var accent_foreground: Color { return Color(red: 0.969, green: 0.98, blue: 0.988) }
    var destructive: Color { return Color(red: 0.498, green: 0.114, blue: 0.114) }
    var destructive_foreground: Color { return Color(red: 0.969, green: 0.98, blue: 0.988) }
    var border: Color { return Color(red: 0.153, green: 0.22, blue: 0.286) }
    var input: Color { return Color(red: 0.153, green: 0.22, blue: 0.286) }
    var ring: Color { return Color(red: 0.125, green: 0.424, blue: 0.922) }
    var general_link_color: Color { return Color(red: 0.118, green: 0.565, blue: 1) }

} 



// -- Light Themes --


struct ZincLight: ThemeProtocol {
    typealias ItemType = WebViewTheme
    var id: WebViewTheme = .zinc
    var background: Color { return Color(red: 1, green: 1, blue: 1) }
    var foreground: Color { return Color(red: 0.051, green: 0.051, blue: 0.059) }
    var card: Color { return Color(red: 1, green: 1, blue: 1) }
    var card_foreground: Color { return Color(red: 0.051, green: 0.051, blue: 0.059) }
    var popover: Color { return Color(red: 1, green: 1, blue: 1) }
    var popover_foreground: Color { return Color(red: 0.051, green: 0.051, blue: 0.059) }
    var primary: Color { return Color(red: 0.094, green: 0.094, blue: 0.098) }
    var primary_foreground: Color { return Color(red: 0.98, green: 0.98, blue: 0.98) }
    var secondary: Color { return Color(red: 0.957, green: 0.957, blue: 0.961) }
    var secondary_foreground: Color { return Color(red: 0.094, green: 0.094, blue: 0.098) }
    var muted: Color { return Color(red: 0.957, green: 0.957, blue: 0.961) }
    var muted_foreground: Color { return Color(red: 0.443, green: 0.443, blue: 0.482) }
    var accent: Color { return Color(red: 0.957, green: 0.957, blue: 0.961) }
    var accent_foreground: Color { return Color(red: 0.094, green: 0.094, blue: 0.098) }
    var destructive: Color { return Color(red: 0.937, green: 0.267, blue: 0.267) }
    var destructive_foreground: Color { return Color(red: 0.98, green: 0.98, blue: 0.98) }
    var border: Color { return Color(red: 0.894, green: 0.894, blue: 0.906) }
    var input: Color { return Color(red: 0.894, green: 0.894, blue: 0.906) }
    var ring: Color { return Color(red: 0.094, green: 0.094, blue: 0.098) }
    var brand: Color { return Color(red: 0.043, green: 0.647, blue: 0.914) }
    var general_link_color: Color { return Color(red: 0.145, green: 0.39, blue: 0.922) }

} 

struct YellowLight: ThemeProtocol {
    typealias ItemType = WebViewTheme
    var id: WebViewTheme = .yellow
    var background: Color { return Color(red: 1, green: 1, blue: 1) }
    var foreground: Color { return Color(red: 0.102, green: 0.086, blue: 0.059) }
    var card: Color { return Color(red: 1, green: 1, blue: 1) }
    var card_foreground: Color { return Color(red: 0.102, green: 0.086, blue: 0.059) }
    var popover: Color { return Color(red: 1, green: 1, blue: 1) }
    var popover_foreground: Color { return Color(red: 0.102, green: 0.086, blue: 0.059) }
    var primary: Color { return Color(red: 0.98, green: 0.8, blue: 0.078) }
    var primary_foreground: Color { return Color(red: 0.2, green: 0.137, blue: 0.02) }
    var secondary: Color { return Color(red: 0.957, green: 0.957, blue: 0.961) }
    var secondary_foreground: Color { return Color(red: 0.106, green: 0.098, blue: 0.09) }
    var muted: Color { return Color(red: 0.957, green: 0.957, blue: 0.961) }
    var muted_foreground: Color { return Color(red: 0.455, green: 0.431, blue: 0.384) }
    var accent: Color { return Color(red: 0.957, green: 0.957, blue: 0.961) }
    var accent_foreground: Color { return Color(red: 0.106, green: 0.098, blue: 0.09) }
    var destructive: Color { return Color(red: 0.937, green: 0.267, blue: 0.267) }
    var destructive_foreground: Color { return Color(red: 0.98, green: 0.98, blue: 0.976) }
    var border: Color { return Color(red: 0.902, green: 0.898, blue: 0.89) }
    var input: Color { return Color(red: 0.902, green: 0.898, blue: 0.89) }
    var ring: Color { return Color(red: 0.102, green: 0.086, blue: 0.059) }
    var brand: Color { return Color(red: 0.043, green: 0.647, blue: 0.914) }
    var general_link_color: Color { return Color(red: 0.145, green: 0.39, blue: 0.922) }

} 

struct VioletLight: ThemeProtocol {
    typealias ItemType = WebViewTheme
    var id: WebViewTheme = .violet
    var background: Color { return Color(red: 1, green: 1, blue: 1) }
    var foreground: Color { return Color(red: 0.082, green: 0.016, blue: 0.18) }
    var card: Color { return Color(red: 1, green: 1, blue: 1) }
    var card_foreground: Color { return Color(red: 0.082, green: 0.016, blue: 0.18) }
    var popover: Color { return Color(red: 1, green: 1, blue: 1) }
    var popover_foreground: Color { return Color(red: 0.082, green: 0.016, blue: 0.18) }
    var primary: Color { return Color(red: 0.482, green: 0.224, blue: 0.929) }
    var primary_foreground: Color { return Color(red: 0.969, green: 0.961, blue: 0.988) }
    var secondary: Color { return Color(red: 0.945, green: 0.949, blue: 0.973) }
    var secondary_foreground: Color { return Color(red: 0.118, green: 0.059, blue: 0.271) }
    var muted: Color { return Color(red: 0.945, green: 0.949, blue: 0.973) }
    var muted_foreground: Color { return Color(red: 0.471, green: 0.463, blue: 0.529) }
    var accent: Color { return Color(red: 0.945, green: 0.949, blue: 0.973) }
    var accent_foreground: Color { return Color(red: 0.118, green: 0.059, blue: 0.271) }
    var destructive: Color { return Color(red: 0.937, green: 0.267, blue: 0.267) }
    var destructive_foreground: Color { return Color(red: 0.969, green: 0.961, blue: 0.988) }
    var border: Color { return Color(red: 0.898, green: 0.902, blue: 0.922) }
    var input: Color { return Color(red: 0.898, green: 0.902, blue: 0.922) }
    var ring: Color { return Color(red: 0.482, green: 0.224, blue: 0.929) }
    var brand: Color { return Color(red: 0.043, green: 0.647, blue: 0.914) }
    var general_link_color: Color { return Color(red: 0.145, green: 0.39, blue: 0.922) }

} 

struct FlusterLight: ThemeProtocol {
    typealias ItemType = WebViewTheme
    var id: WebViewTheme = .fluster
    var background: Color { return Color(red: 1, green: 1, blue: 1) }
    var foreground: Color { return Color(red: 0.043, green: 0.082, blue: 0.153) }
    var card: Color { return Color(red: 1, green: 1, blue: 1) }
    var card_foreground: Color { return Color(red: 0.043, green: 0.082, blue: 0.153) }
    var popover: Color { return Color(red: 1, green: 1, blue: 1) }
    var popover_foreground: Color { return Color(red: 0.043, green: 0.082, blue: 0.153) }
    var primary: Color { return Color(red: 0.043, green: 0.647, blue: 0.914) }
    var primary_foreground: Color { return Color(red: 0.969, green: 0.98, blue: 0.988) }
    var secondary: Color { return Color(red: 0.941, green: 0.961, blue: 0.976) }
    var secondary_foreground: Color { return Color(red: 0.063, green: 0.09, blue: 0.165) }
    var muted: Color { return Color(red: 0.941, green: 0.961, blue: 0.976) }
    var muted_foreground: Color { return Color(red: 0.471, green: 0.53, blue: 0.608) }
    var accent: Color { return Color(red: 0.941, green: 0.961, blue: 0.976) }
    var accent_foreground: Color { return Color(red: 0.063, green: 0.09, blue: 0.165) }
    var destructive: Color { return Color(red: 0.937, green: 0.267, blue: 0.267) }
    var destructive_foreground: Color { return Color(red: 0.969, green: 0.98, blue: 0.988) }
    var border: Color { return Color(red: 0.902, green: 0.922, blue: 0.941) }
    var input: Color { return Color(red: 0.902, green: 0.922, blue: 0.941) }
    var ring: Color { return Color(red: 0.22, green: 0.749, blue: 0.506) }
    var brand: Color { return Color(red: 0.043, green: 0.647, blue: 0.914) }
    var general_link_color: Color { return Color(red: 0.145, green: 0.39, blue: 0.922) }

} 

struct StoneLight: ThemeProtocol {
    typealias ItemType = WebViewTheme
    var id: WebViewTheme = .stone
    var background: Color { return Color(red: 1, green: 1, blue: 1) }
    var foreground: Color { return Color(red: 0.102, green: 0.086, blue: 0.059) }
    var card: Color { return Color(red: 1, green: 1, blue: 1) }
    var card_foreground: Color { return Color(red: 0.102, green: 0.086, blue: 0.059) }
    var popover: Color { return Color(red: 1, green: 1, blue: 1) }
    var popover_foreground: Color { return Color(red: 0.102, green: 0.086, blue: 0.059) }
    var primary: Color { return Color(red: 0.106, green: 0.098, blue: 0.09) }
    var primary_foreground: Color { return Color(red: 0.98, green: 0.98, blue: 0.976) }
    var secondary: Color { return Color(red: 0.957, green: 0.957, blue: 0.961) }
    var secondary_foreground: Color { return Color(red: 0.106, green: 0.098, blue: 0.09) }
    var muted: Color { return Color(red: 0.957, green: 0.957, blue: 0.961) }
    var muted_foreground: Color { return Color(red: 0.455, green: 0.431, blue: 0.384) }
    var accent: Color { return Color(red: 0.957, green: 0.957, blue: 0.961) }
    var accent_foreground: Color { return Color(red: 0.106, green: 0.098, blue: 0.09) }
    var destructive: Color { return Color(red: 0.937, green: 0.267, blue: 0.267) }
    var destructive_foreground: Color { return Color(red: 0.98, green: 0.98, blue: 0.976) }
    var border: Color { return Color(red: 0.902, green: 0.898, blue: 0.89) }
    var input: Color { return Color(red: 0.902, green: 0.898, blue: 0.89) }
    var ring: Color { return Color(red: 0.102, green: 0.086, blue: 0.059) }
    var brand: Color { return Color(red: 0.043, green: 0.647, blue: 0.914) }
    var general_link_color: Color { return Color(red: 0.145, green: 0.39, blue: 0.922) }

} 

struct SlateLight: ThemeProtocol {
    typealias ItemType = WebViewTheme
    var id: WebViewTheme = .slate
    var background: Color { return Color(red: 1, green: 1, blue: 1) }
    var foreground: Color { return Color(red: 0.043, green: 0.082, blue: 0.153) }
    var card: Color { return Color(red: 1, green: 1, blue: 1) }
    var card_foreground: Color { return Color(red: 0.043, green: 0.082, blue: 0.153) }
    var popover: Color { return Color(red: 1, green: 1, blue: 1) }
    var popover_foreground: Color { return Color(red: 0.043, green: 0.082, blue: 0.153) }
    var primary: Color { return Color(red: 0.063, green: 0.09, blue: 0.165) }
    var primary_foreground: Color { return Color(red: 0.969, green: 0.98, blue: 0.988) }
    var secondary: Color { return Color(red: 0.941, green: 0.961, blue: 0.976) }
    var secondary_foreground: Color { return Color(red: 0.063, green: 0.09, blue: 0.165) }
    var muted: Color { return Color(red: 0.941, green: 0.961, blue: 0.976) }
    var muted_foreground: Color { return Color(red: 0.471, green: 0.53, blue: 0.608) }
    var accent: Color { return Color(red: 0.941, green: 0.961, blue: 0.976) }
    var accent_foreground: Color { return Color(red: 0.063, green: 0.09, blue: 0.165) }
    var destructive: Color { return Color(red: 0.937, green: 0.267, blue: 0.267) }
    var destructive_foreground: Color { return Color(red: 0.969, green: 0.98, blue: 0.988) }
    var border: Color { return Color(red: 0.902, green: 0.922, blue: 0.941) }
    var input: Color { return Color(red: 0.902, green: 0.922, blue: 0.941) }
    var ring: Color { return Color(red: 0.043, green: 0.082, blue: 0.153) }
    var brand: Color { return Color(red: 0.043, green: 0.647, blue: 0.914) }
    var general_link_color: Color { return Color(red: 0.145, green: 0.39, blue: 0.922) }

} 

struct RoseLight: ThemeProtocol {
    typealias ItemType = WebViewTheme
    var id: WebViewTheme = .rose
    var background: Color { return Color(red: 1, green: 1, blue: 1) }
    var foreground: Color { return Color(red: 0.051, green: 0.051, blue: 0.059) }
    var card: Color { return Color(red: 1, green: 1, blue: 1) }
    var card_foreground: Color { return Color(red: 0.051, green: 0.051, blue: 0.059) }
    var popover: Color { return Color(red: 1, green: 1, blue: 1) }
    var popover_foreground: Color { return Color(red: 0.051, green: 0.051, blue: 0.059) }
    var primary: Color { return Color(red: 0.882, green: 0.114, blue: 0.278) }
    var primary_foreground: Color { return Color(red: 0.996, green: 0.953, blue: 0.961) }
    var secondary: Color { return Color(red: 0.957, green: 0.957, blue: 0.961) }
    var secondary_foreground: Color { return Color(red: 0.094, green: 0.094, blue: 0.098) }
    var muted: Color { return Color(red: 0.957, green: 0.957, blue: 0.961) }
    var muted_foreground: Color { return Color(red: 0.443, green: 0.443, blue: 0.482) }
    var accent: Color { return Color(red: 0.957, green: 0.957, blue: 0.961) }
    var accent_foreground: Color { return Color(red: 0.094, green: 0.094, blue: 0.098) }
    var destructive: Color { return Color(red: 0.937, green: 0.267, blue: 0.267) }
    var destructive_foreground: Color { return Color(red: 0.98, green: 0.98, blue: 0.98) }
    var border: Color { return Color(red: 0.894, green: 0.894, blue: 0.906) }
    var input: Color { return Color(red: 0.894, green: 0.894, blue: 0.906) }
    var ring: Color { return Color(red: 0.882, green: 0.114, blue: 0.278) }
    var brand: Color { return Color(red: 0.043, green: 0.647, blue: 0.914) }
    var general_link_color: Color { return Color(red: 0.145, green: 0.39, blue: 0.922) }

} 

struct RedLight: ThemeProtocol {
    typealias ItemType = WebViewTheme
    var id: WebViewTheme = .red
    var background: Color { return Color(red: 1, green: 1, blue: 1) }
    var foreground: Color { return Color(red: 0.039, green: 0.039, blue: 0.039) }
    var card: Color { return Color(red: 1, green: 1, blue: 1) }
    var card_foreground: Color { return Color(red: 0.039, green: 0.039, blue: 0.039) }
    var popover: Color { return Color(red: 1, green: 1, blue: 1) }
    var popover_foreground: Color { return Color(red: 0.039, green: 0.039, blue: 0.039) }
    var primary: Color { return Color(red: 0.863, green: 0.149, blue: 0.153) }
    var primary_foreground: Color { return Color(red: 0.996, green: 0.953, blue: 0.961) }
    var secondary: Color { return Color(red: 0.961, green: 0.961, blue: 0.961) }
    var secondary_foreground: Color { return Color(red: 0.09, green: 0.09, blue: 0.09) }
    var muted: Color { return Color(red: 0.961, green: 0.961, blue: 0.961) }
    var muted_foreground: Color { return Color(red: 0.451, green: 0.451, blue: 0.451) }
    var accent: Color { return Color(red: 0.961, green: 0.961, blue: 0.961) }
    var accent_foreground: Color { return Color(red: 0.09, green: 0.09, blue: 0.09) }
    var destructive: Color { return Color(red: 0.937, green: 0.267, blue: 0.267) }
    var destructive_foreground: Color { return Color(red: 0.98, green: 0.98, blue: 0.98) }
    var border: Color { return Color(red: 0.898, green: 0.898, blue: 0.898) }
    var input: Color { return Color(red: 0.898, green: 0.898, blue: 0.898) }
    var ring: Color { return Color(red: 0.863, green: 0.149, blue: 0.153) }
    var brand: Color { return Color(red: 0.043, green: 0.647, blue: 0.914) }
    var general_link_color: Color { return Color(red: 0.145, green: 0.39, blue: 0.922) }

} 

struct OrangeLight: ThemeProtocol {
    typealias ItemType = WebViewTheme
    var id: WebViewTheme = .orange
    var background: Color { return Color(red: 1, green: 1, blue: 1) }
    var foreground: Color { return Color(red: 0.102, green: 0.086, blue: 0.059) }
    var card: Color { return Color(red: 1, green: 1, blue: 1) }
    var card_foreground: Color { return Color(red: 0.102, green: 0.086, blue: 0.059) }
    var popover: Color { return Color(red: 1, green: 1, blue: 1) }
    var popover_foreground: Color { return Color(red: 0.102, green: 0.086, blue: 0.059) }
    var primary: Color { return Color(red: 0.973, green: 0.451, blue: 0.082) }
    var primary_foreground: Color { return Color(red: 0.98, green: 0.98, blue: 0.976) }
    var secondary: Color { return Color(red: 0.957, green: 0.957, blue: 0.961) }
    var secondary_foreground: Color { return Color(red: 0.106, green: 0.098, blue: 0.09) }
    var muted: Color { return Color(red: 0.957, green: 0.957, blue: 0.961) }
    var muted_foreground: Color { return Color(red: 0.455, green: 0.431, blue: 0.384) }
    var accent: Color { return Color(red: 0.957, green: 0.957, blue: 0.961) }
    var accent_foreground: Color { return Color(red: 0.106, green: 0.098, blue: 0.09) }
    var destructive: Color { return Color(red: 0.937, green: 0.267, blue: 0.267) }
    var destructive_foreground: Color { return Color(red: 0.98, green: 0.98, blue: 0.976) }
    var border: Color { return Color(red: 0.902, green: 0.898, blue: 0.89) }
    var input: Color { return Color(red: 0.902, green: 0.898, blue: 0.89) }
    var ring: Color { return Color(red: 0.973, green: 0.451, blue: 0.082) }
    var brand: Color { return Color(red: 0.043, green: 0.647, blue: 0.914) }
    var general_link_color: Color { return Color(red: 0.145, green: 0.39, blue: 0.922) }

} 

struct NeutralLight: ThemeProtocol {
    typealias ItemType = WebViewTheme
    var id: WebViewTheme = .neutral
    var background: Color { return Color(red: 1, green: 1, blue: 1) }
    var foreground: Color { return Color(red: 0.039, green: 0.039, blue: 0.039) }
    var card: Color { return Color(red: 1, green: 1, blue: 1) }
    var card_foreground: Color { return Color(red: 0.039, green: 0.039, blue: 0.039) }
    var popover: Color { return Color(red: 1, green: 1, blue: 1) }
    var popover_foreground: Color { return Color(red: 0.039, green: 0.039, blue: 0.039) }
    var primary: Color { return Color(red: 0.09, green: 0.09, blue: 0.09) }
    var primary_foreground: Color { return Color(red: 0.98, green: 0.98, blue: 0.98) }
    var secondary: Color { return Color(red: 0.961, green: 0.961, blue: 0.961) }
    var secondary_foreground: Color { return Color(red: 0.09, green: 0.09, blue: 0.09) }
    var muted: Color { return Color(red: 0.961, green: 0.961, blue: 0.961) }
    var muted_foreground: Color { return Color(red: 0.451, green: 0.451, blue: 0.451) }
    var accent: Color { return Color(red: 0.961, green: 0.961, blue: 0.961) }
    var accent_foreground: Color { return Color(red: 0.09, green: 0.09, blue: 0.09) }
    var destructive: Color { return Color(red: 0.937, green: 0.267, blue: 0.267) }
    var destructive_foreground: Color { return Color(red: 0.98, green: 0.98, blue: 0.98) }
    var border: Color { return Color(red: 0.898, green: 0.898, blue: 0.898) }
    var input: Color { return Color(red: 0.898, green: 0.898, blue: 0.898) }
    var ring: Color { return Color(red: 0.039, green: 0.039, blue: 0.039) }
    var brand: Color { return Color(red: 0.043, green: 0.647, blue: 0.914) }
    var general_link_color: Color { return Color(red: 0.145, green: 0.39, blue: 0.922) }

} 

struct GreenLight: ThemeProtocol {
    typealias ItemType = WebViewTheme
    var id: WebViewTheme = .green
    var background: Color { return Color(red: 1, green: 1, blue: 1) }
    var foreground: Color { return Color(red: 0.051, green: 0.051, blue: 0.059) }
    var card: Color { return Color(red: 1, green: 1, blue: 1) }
    var card_foreground: Color { return Color(red: 0.051, green: 0.051, blue: 0.059) }
    var popover: Color { return Color(red: 1, green: 1, blue: 1) }
    var popover_foreground: Color { return Color(red: 0.051, green: 0.051, blue: 0.059) }
    var primary: Color { return Color(red: 0.086, green: 0.64, blue: 0.286) }
    var primary_foreground: Color { return Color(red: 0.996, green: 0.953, blue: 0.961) }
    var secondary: Color { return Color(red: 0.957, green: 0.957, blue: 0.961) }
    var secondary_foreground: Color { return Color(red: 0.094, green: 0.094, blue: 0.098) }
    var muted: Color { return Color(red: 0.957, green: 0.957, blue: 0.961) }
    var muted_foreground: Color { return Color(red: 0.443, green: 0.443, blue: 0.482) }
    var accent: Color { return Color(red: 0.957, green: 0.957, blue: 0.961) }
    var accent_foreground: Color { return Color(red: 0.094, green: 0.094, blue: 0.098) }
    var destructive: Color { return Color(red: 0.937, green: 0.267, blue: 0.267) }
    var destructive_foreground: Color { return Color(red: 0.98, green: 0.98, blue: 0.98) }
    var border: Color { return Color(red: 0.894, green: 0.894, blue: 0.906) }
    var input: Color { return Color(red: 0.894, green: 0.894, blue: 0.906) }
    var ring: Color { return Color(red: 0.086, green: 0.64, blue: 0.286) }
    var brand: Color { return Color(red: 0.043, green: 0.647, blue: 0.914) }
    var general_link_color: Color { return Color(red: 0.145, green: 0.39, blue: 0.922) }

} 

struct GrayLight: ThemeProtocol {
    typealias ItemType = WebViewTheme
    var id: WebViewTheme = .gray
    var background: Color { return Color(red: 1, green: 1, blue: 1) }
    var foreground: Color { return Color(red: 0.082, green: 0.016, blue: 0.18) }
    var card: Color { return Color(red: 1, green: 1, blue: 1) }
    var card_foreground: Color { return Color(red: 0.082, green: 0.016, blue: 0.18) }
    var popover: Color { return Color(red: 1, green: 1, blue: 1) }
    var popover_foreground: Color { return Color(red: 0.082, green: 0.016, blue: 0.18) }
    var primary: Color { return Color(red: 0.118, green: 0.059, blue: 0.271) }
    var primary_foreground: Color { return Color(red: 0.969, green: 0.961, blue: 0.988) }
    var secondary: Color { return Color(red: 0.945, green: 0.949, blue: 0.973) }
    var secondary_foreground: Color { return Color(red: 0.118, green: 0.059, blue: 0.271) }
    var muted: Color { return Color(red: 0.945, green: 0.949, blue: 0.973) }
    var muted_foreground: Color { return Color(red: 0.471, green: 0.463, blue: 0.529) }
    var accent: Color { return Color(red: 0.945, green: 0.949, blue: 0.973) }
    var accent_foreground: Color { return Color(red: 0.118, green: 0.059, blue: 0.271) }
    var destructive: Color { return Color(red: 0.937, green: 0.267, blue: 0.267) }
    var destructive_foreground: Color { return Color(red: 0.969, green: 0.961, blue: 0.988) }
    var border: Color { return Color(red: 0.898, green: 0.902, blue: 0.922) }
    var input: Color { return Color(red: 0.898, green: 0.902, blue: 0.922) }
    var ring: Color { return Color(red: 0.082, green: 0.016, blue: 0.18) }
    var brand: Color { return Color(red: 0.043, green: 0.647, blue: 0.914) }
    var general_link_color: Color { return Color(red: 0.145, green: 0.39, blue: 0.922) }

} 

struct BlueLight: ThemeProtocol {
    typealias ItemType = WebViewTheme
    var id: WebViewTheme = .blue
    var background: Color { return Color(red: 1, green: 1, blue: 1) }
    var foreground: Color { return Color(red: 0.043, green: 0.082, blue: 0.153) }
    var card: Color { return Color(red: 1, green: 1, blue: 1) }
    var card_foreground: Color { return Color(red: 0.043, green: 0.082, blue: 0.153) }
    var popover: Color { return Color(red: 1, green: 1, blue: 1) }
    var popover_foreground: Color { return Color(red: 0.043, green: 0.082, blue: 0.153) }
    var primary: Color { return Color(red: 0.141, green: 0.388, blue: 0.922) }
    var primary_foreground: Color { return Color(red: 0.969, green: 0.98, blue: 0.988) }
    var secondary: Color { return Color(red: 0.941, green: 0.961, blue: 0.976) }
    var secondary_foreground: Color { return Color(red: 0.063, green: 0.09, blue: 0.165) }
    var muted: Color { return Color(red: 0.941, green: 0.961, blue: 0.976) }
    var muted_foreground: Color { return Color(red: 0.471, green: 0.53, blue: 0.608) }
    var accent: Color { return Color(red: 0.941, green: 0.961, blue: 0.976) }
    var accent_foreground: Color { return Color(red: 0.063, green: 0.09, blue: 0.165) }
    var destructive: Color { return Color(red: 0.937, green: 0.267, blue: 0.267) }
    var destructive_foreground: Color { return Color(red: 0.969, green: 0.98, blue: 0.988) }
    var border: Color { return Color(red: 0.902, green: 0.922, blue: 0.941) }
    var input: Color { return Color(red: 0.902, green: 0.922, blue: 0.941) }
    var ring: Color { return Color(red: 0.141, green: 0.388, blue: 0.922) }
    var brand: Color { return Color(red: 0.043, green: 0.647, blue: 0.914) }
    var general_link_color: Color { return Color(red: 0.145, green: 0.39, blue: 0.922) }
} 


func getTheme(themeName: WebViewTheme, darkMode: Bool) -> any ThemeProtocol {
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
