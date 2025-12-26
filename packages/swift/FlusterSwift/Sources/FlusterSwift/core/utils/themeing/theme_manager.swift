import Combine
import SwiftUI

@Observable
@MainActor
public class ThemeManager {
  /// The selected theme.
  public var theme: any ThemeProtocol

  /// Create a new theme manager instance.
  ///
  /// - Parameters:
  ///   - initialTheme: The theme that is initially selected.
  public init(initialTheme: any ThemeProtocol) {
    self.theme = initialTheme
  }
}
