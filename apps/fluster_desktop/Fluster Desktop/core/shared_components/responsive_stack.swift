//
//  responsive_stack.swift
//  Fluster
//
//  Created by Andrew on 2/8/26.
//

import SwiftUI

struct HeightPreferenceKey: PreferenceKey {
  static var defaultValue: CGFloat = 0
  static func reduce(value: inout CGFloat, nextValue: () -> CGFloat) {
    value = nextValue()
  }
}

public struct ResponsiveStack<Content: View>: View {
    let threshold: CGFloat
    let content: () -> Content
    
    @State private var currentWidth: CGFloat = 0
    @State private var dynamicHeight: CGFloat = .zero

    public var body: some View {
        // 1. External GeometryReader to measure available width
        ZStack {
            GeometryReader { proxy in
                Color.clear.onAppear {
                    currentWidth = proxy.size.width
                }
                .onChange(of: proxy.size.width) {
                    currentWidth = proxy.size.width
                }
            }
            
            // 2. Choose layout based on the measured width
            let layout = currentWidth > threshold ? AnyLayout(HStackLayout()) : AnyLayout(VStackLayout())

            layout {
                content()
            }
            // 3. Measure resulting height of the chosen layout
            .background(
                GeometryReader { geo in
                    Color.clear.preference(key: HeightPreferenceKey.self, value: geo.size.height)
                }
            )
        }
        .onPreferenceChange(HeightPreferenceKey.self) { dynamicHeight = $0 }
        // 4. Set the frame so the parent knows the height
        .frame(height: dynamicHeight)
    }
}
