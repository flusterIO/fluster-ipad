//
//  webview_font_size.swift
//  Fluster
//
//  Created by Andrew on 11/21/25.
//

public enum WebviewFontSize: String, CaseIterable {
  case small, base, large, xl, xxl
  func cssClass() -> String {
    switch self {
      case .small:
        return "prose-sm"
      case .base:
        return "prose-base"
      case .large:
        return "prose-lg"
      case .xl:
        return "prose-xl"
      case .xxl:
        return "prose-2xl"
    }
  }
}
