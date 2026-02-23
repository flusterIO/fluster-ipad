//
//  command_palette_item_view.swift
//  Fluster Desktop
//
//  Created by Andrew on 1/14/26.
//

import SwiftUI

struct CommandPaletteItemView: View {
  @Environment(\.colorScheme) var colorScheme
  let command: CommandPaletteItem
  let idx: Int
  @Binding var focusedIndex: Int
  let onCommandSelected: (CommandPaletteItem) -> Void
  @FocusState private var isFocused: Bool
  var body: some View {
    Button(action: { onCommandSelected(command) }) {
      HStack {
        Label(
          title: {
            Text(command.title)
              .foregroundStyle(
                idx == focusedIndex
                  ? (colorScheme == .dark ? Color.white : Color.black) : .secondary)
          },
          icon: {
            Image(systemName: command.icon)
              .foregroundStyle(idx == focusedIndex ? Color.accentColor : .secondary)
          }
        )
        .foregroundStyle(.primary)
        Spacer()
      }
    }
    .buttonStyle(.borderless)
    .focused($isFocused)
    .padding(.vertical, 4)
    .clipShape(.rect(cornerRadius: 0))
    .background(RoundedRectangle(cornerRadius: 0).fill(Color.accentColor.opacity(0.02)))
  }
}

#Preview {
  CommandPaletteItemView(
    command: CommandPaletteRoot(), idx: 0, focusedIndex: .constant(0),
    onCommandSelected: { cmd in
    }
  )
  CommandPaletteItemView(
    command: CommandPaletteRoot(), idx: 1, focusedIndex: .constant(0),
    onCommandSelected: { cmd in
    }
  )
}
