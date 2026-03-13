import SwiftUI

extension Binding {
  /// Provides a default value for an optional binding, converting it to a non-optional binding.
  func withDefault<T>(_ defaultValue: T) -> Binding<T> where Value == T? {
    return Binding<T>(
      get: { self.wrappedValue ?? defaultValue },
      set: { newValue in self.wrappedValue = newValue }
    )
  }
  static func toOptional<T>(_ binding: Binding<T>) -> Binding<T?> {
    Binding<T?>(
      get: { binding.wrappedValue },
      set: { newValue in
        // Only update if the newValue is not nil
        // to avoid crashing the non-optional source
        if let newValue = newValue {
          binding.wrappedValue = newValue
        }
      }
    )
  }
}
