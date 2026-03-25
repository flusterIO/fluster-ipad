/// Deprecated... moved everythng to Rust
extension CodeEditorTheme {
  public func toThemeLabel() -> String {
    switch self {
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
  public func isDark() -> Bool {
    let darkThemes: [Self] = [
      .aura, .dracula, .xcodeDark, .materialDark, .solarizedDark, .tokyoNight, .tokyoNightStorm
    ]
    return darkThemes.contains(self)
  }
}
