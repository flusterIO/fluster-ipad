//
//  app_theme.swift
//  Fluster Desktop
//
//  Created by Andrew on 1/15/26.
//

import SwiftUI

enum AppTheme: String, CaseIterable {
  case light = "Light"
  case dark = "Dark"

  var colorScheme: ColorScheme? {
    switch self {
      case .light: return .light
      case .dark: return .dark
    }
  }
}
