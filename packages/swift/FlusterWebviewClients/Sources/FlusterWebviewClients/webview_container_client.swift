
import FlusterData
import SwiftUI
import SwiftData

public struct WebviewContainerClient {
  public static func setColorScheme(
    colorScheme: ColorScheme, evaluateJavaScript: @escaping EvalJavascriptFunc
  ) async throws {
      do {
        try await evaluateJavaScript(
          """
          if (window && ("setDarkMode" in window)) {
              window.setDarkMode(\(colorScheme == .dark ? "true" : "false"))
          } else {
              console.log("Could not webview container functions")
          }
          """)
      } catch {
        print("Error setting color scheme: \(error.localizedDescription)")
      }
  }
}
