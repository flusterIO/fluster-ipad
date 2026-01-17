//
//  command_palette_container_view.swift
//  Fluster Desktop
//
//  Created by Andrew on 1/14/26.
//

import Combine
import SwiftUI

struct CommandPaletteContainerView: View {
  @State private var open: Bool = false
  @State private var searchText: String = ""
  @Environment(\.modelContext) private var modelContext

  @FocusState private var searchFieldFocused: Bool

  @State private var tree: [CommandPaletteItem] = [CommandPaletteRoot()]

  var body: some View {
    let filteredCommands = Binding<[CommandPaletteItem]>(
      get: {
        return searchText.isEmpty
          ? (tree.last!.hasChildren ? tree.last!.children(modelContext: modelContext) : [])
          : (tree.last!.hasChildren
            ? tree.last!.children(modelContext: modelContext).filter {
              $0.title.localizedCaseInsensitiveContains(searchText)
            }
            : [])
      },
      set: { newValue in
        return
      }
    )
    ZStack {
      // Main content placeholder
      Color.clear
        .frame(maxWidth: .infinity, maxHeight: .infinity)

      if open {
        CommandPaletteView(
          searchText: $searchText,
          results: filteredCommands,
          onClose: {
            open = false
            searchText = ""
            tree = [CommandPaletteRoot()]
          },
          tree: $tree,
          onCommandSelected: { command in
            if command.hasChildren {
              tree.append(command)
            } else {
              executeCommandPaletteAction(action: command.id)
              open = false
              tree = [CommandPaletteRoot()]
              searchText = ""
            }
          }
        )
        .transition(.opacity.combined(with: .scale))
        .zIndex(10)
        .onAppear { searchFieldFocused = true }
        .onChange(of: open) {
          if open { searchFieldFocused = true }
        }
      }
    }
    .onAppear {
      // Register global keyboard shortcut
      #if os(macOS)
        NSEvent.addLocalMonitorForEvents(matching: .keyDown) { event in
          if event.modifierFlags.contains([.command, .shift])
            && event.charactersIgnoringModifiers == "P"
          {
            open.toggle()
            return nil
          }
          return event
        }
      #endif
    }
    .onReceive(NotificationCenter.default.publisher(for: NSNotification.Name("ShowCommandPalette")))
    { _ in
      open = true
    }
    .background(CommandPaletteKeyboardShortcut(open: $open))
  }
}

private struct CommandPaletteView: View {
  @Binding var searchText: String
  @Binding var results: [CommandPaletteItem]
  let onClose: () -> Void
  @Binding var tree: [CommandPaletteItem]
  let onCommandSelected: (CommandPaletteItem) -> Void
  @FocusState private var searchFieldFocused: Bool
  @State private var focusedIndex: Int = 0
  @Environment(\.colorScheme) private var colorScheme: ColorScheme

  var body: some View {
    ZStack(alignment: .top) {
      Color.black.opacity(colorScheme == .dark ? 0.25 : 0.125)
        .ignoresSafeArea()
        .onTapGesture { onClose() }
      VStack(spacing: 0) {
        HStack {
          Image(systemName: "magnifyingglass")
            .foregroundStyle(.secondary)
          CommandPaletteInputViewMac(
            text: $searchText,
            onBackspace: {
              if searchText.isEmpty && tree.count > 1 {
                let _ = tree.popLast()
              }
            },
            onTab: {
              incrementFocus()
            },
            onBackTab: {
              decrementFocus()
            },
            onEnter: {
                if focusedIndex < results.count {
                    let focusedItem = results[focusedIndex]
                    onCommandSelected(focusedItem)
                } else {
                    focusedIndex = 0
                }
            },
            onDownArrow: {
              incrementFocus()
            },
            onUpArrow: {
              decrementFocus()
            }
          )
          .onChange(
            of: searchText,
            {
              focusedIndex = 0
            }
          )
          .onChange(
            of: tree,
            {
              focusedIndex = 0
            }
          )
          .textFieldStyle(.plain)
          .focused($searchFieldFocused)
          .onAppear { searchFieldFocused = true }
        }
        .padding()
        Divider()
        ScrollView {
          VStack(alignment: .leading, spacing: 0) {
            ForEach(Array(results.enumerated()), id: \.offset) { idx, command in
              CommandPaletteItemView(
                command: command, idx: idx, focusedIndex: $focusedIndex,
                onCommandSelected: onCommandSelected)
            }
          }
        }
        .frame(maxHeight: 180)
      }
      .background(colorScheme == .dark ? .black.opacity(0.85) : .white)
      .cornerRadius(18)
      .shadow(radius: 20)
      .frame(maxWidth: 768)
      .padding(.top, 100)
    }
    .onAppear { searchFieldFocused = true }
    .onExitCommand {
      onClose()
    }
  }

  func incrementFocus() {
    if focusedIndex < results.count - 1 {
      focusedIndex += 1
    } else {
      focusedIndex = 0
    }
  }
  func decrementFocus() {
    if focusedIndex > 0 {
      focusedIndex -= 1
    } else {
      focusedIndex = results.count - 1
    }
  }
}

private struct CommandPaletteKeyboardShortcut: View {
  @Binding var open: Bool
  var body: some View {
    Color.clear
      .keyboardShortcut(
        KeyboardShortcut(
          KeyEquivalent(Character("P")), modifiers: [.shift], localization: .automatic))
  }
}

#Preview {
  CommandPaletteContainerView()
}
