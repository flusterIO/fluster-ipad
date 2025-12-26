//
//  color_scheme_selection.swift
//  Fluster
//
//  Created by Andrew on 11/17/25.
//

import SwiftUI

enum ColorSchemeSelection: String, Codable, CaseIterable {
  case dark, light
}

func getColorScheme(selected: ColorSchemeSelection, systemScheme: ColorScheme) -> ColorScheme {
  if selected == .dark {
    return .dark
  }
  if selected == .light {
    return .light
  }
  return systemScheme
}
