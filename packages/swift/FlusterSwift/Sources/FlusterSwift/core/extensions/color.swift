//
//  File.swift
//  FlusterSwift
//
//  Created by Andrew on 3/25/26.
//

import SwiftUI

extension Color {
  init(hex: String) {
    let scanner = Scanner(string: hex)
    scanner.scanLocation = 0
    var rgbValue: UInt64 = 0
    scanner.scanString("#", into: nil)  // Remove the '#' if present
    scanner.scanHexInt64(&rgbValue)

    let r = (rgbValue & 0xff0000) >> 16
    let g = (rgbValue & 0xff00) >> 8
    let b = rgbValue & 0xff

    self.init(red: Double(r) / 255.0, green: Double(g) / 255.0, blue: Double(b) / 255.0)
  }
  public init?(cssHSL: String) {
    let parts =
      cssHSL
      .split(separator: " ")
      .map { $0.trimmingCharacters(in: .whitespacesAndNewlines) }

    guard parts.count >= 3 else { return nil }

    // Extract Hue (0-360)
    let rawHue = parts[0].replacingOccurrences(of: "°", with: "")
    guard let hDeg = Double(rawHue) else { return nil }
    let h = hDeg / 360.0

    // Extract Saturation and Lightness (0-100)
    let rawSat = parts[1].replacingOccurrences(of: "%", with: "")
    let rawLight = parts[2].replacingOccurrences(of: "%", with: "")

    guard let sPct = Double(rawSat), let lPct = Double(rawLight) else { return nil }

    let s = sPct / 100.0
    let l = lPct / 100.0

    // Convert HSL to HSB
    let brightness = l + s * min(l, 1 - l)
    let saturation = brightness == 0 ? 0.0 : 2.0 * (1.0 - (l / brightness))

    // Handle optional alpha channel (e.g., "213 39% 40% / 0.5" or "213 39% 40% / 50%")
    var alpha: Double = 1.0
    if parts.count >= 5 && parts[3] == "/" {
      let rawAlpha = parts[4].replacingOccurrences(of: "%", with: "")
      if let aVal = Double(rawAlpha) {
        alpha = parts[4].contains("%") ? aVal / 100.0 : aVal
      }
    }

    self.init(hue: h, saturation: saturation, brightness: brightness, opacity: alpha)
  }
}
