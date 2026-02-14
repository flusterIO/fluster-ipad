import Combine
import SwiftUI

#if os(iOS)
struct OrientationDetector: ViewModifier {
  @Binding var orientation: UIDeviceOrientation

  func body(content: Content) -> some View {
    content
      .onReceive(
        NotificationCenter.default.publisher(
          for: UIDevice.orientationDidChangeNotification
        )
      ) { _ in
        orientation = UIDevice.current.orientation
      }
  }
}
struct DisableAnimationsViewModifier: ViewModifier {
  func body(content: Content) -> some View {
    content.transaction { transaction in
      transaction.animation = nil
      transaction.disablesAnimations = true
    }
  }
}

struct LogRerender: ViewModifier {
  let msg: String?
  func body(content: Content) -> some View {
    content
      .onChange(
        of: 0,
        initial: true,
        {
          print(msg == nil ? "View has rendered" : msg)
        }
      )
  }
}

extension View {
  public func detectOrientation(orientation: Binding<UIDeviceOrientation>)
    -> some View
  {
    modifier(OrientationDetector(orientation: orientation))
  }
  public func disableAnimations() -> some View {
    modifier(DisableAnimationsViewModifier())
  }
  /// Will print a simple log every tme the view renders.
  public func logRerender(msg: String? = nil) -> some View {
    modifier(LogRerender(msg: msg))
  }

  func roundedCornerWithBorder(
    lineWidth: CGFloat,
    borderColor: Color,
    radius: CGFloat,
  ) -> some View {
    self.overlay(
      RoundedRectangle(cornerRadius: radius)
        .stroke(borderColor, lineWidth: lineWidth)
    )
  }
}
#endif
