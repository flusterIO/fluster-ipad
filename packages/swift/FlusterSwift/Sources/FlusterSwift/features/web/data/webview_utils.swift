import SwiftUI

#if os(iOS)
public func getViewportHeight() -> CGFloat {
  if let h = UIScreen.current?.bounds.height {
    return h
  }
  return 0
}
#endif
