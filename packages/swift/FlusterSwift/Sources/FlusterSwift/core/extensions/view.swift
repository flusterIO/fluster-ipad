import Combine
import SwiftUI

struct OrientationDetector: ViewModifier {
    @Binding var orientation: UIDeviceOrientation

    func body(content: Content) -> some View {
        content
            .onReceive(
                NotificationCenter.default.publisher(for: UIDevice.orientationDidChangeNotification)
            ) { _ in
                orientation = UIDevice.current.orientation
            }
    }
}

public extension View {
    func detectOrientation(orientation: Binding<UIDeviceOrientation>) -> some View {
        modifier(OrientationDetector(orientation: orientation))
    }
}
