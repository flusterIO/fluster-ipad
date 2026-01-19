//
//  quick_action_item_view.swift
//  Fluster
//
//  Created by Andrew on 1/18/26.
//

import SwiftUI

struct QuickActionItemView: View {
  let item: QuickAction
  @Environment(AppState.self) private var appState: AppState
  var body: some View {
    VStack(spacing: 4) {
      Image(systemName: item.icon)
        .font(.largeTitle)
        .padding(6)
        .foregroundStyle(.white)
        .background(RoundedRectangle(cornerRadius: 12).fill(item.color))
      Text(item.label)
        .lineLimit(1)
        .font(.headline)
    }
    .frame(maxWidth: .infinity)
    .frame(height: 100)
    .padding(.vertical)
    .padding(.horizontal, 32)
    .glassEffect(in: .rect(cornerRadius: 12))
    .onTapGesture {
      item.action(appState)
    }
  }
}

#Preview {
  QuickActionItemView(item: quickActions.first!)
}
