import SwiftUI
import Combine


@Observable
@MainActor
public class ThemeManager {
    /// The selected theme.
    var theme: ThemeProtocol

    /// Create a new theme manager instance.
    ///
    /// - Parameters:
    ///   - initialTheme: The theme that is initially selected.
    public init(initialTheme: ThemeProtocol) {
        self.theme = initialTheme
    }
}
