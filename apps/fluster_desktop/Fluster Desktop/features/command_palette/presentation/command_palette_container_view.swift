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
  @Environment(AppState.self) private var appState: AppState
  @State private var commandPaletteNavigation: CommandPaletteSecondaryView? = nil

  @FocusState private var searchFieldFocused: Bool

  @State private var tree: [CommandPaletteItem] = [CommandPaletteRoot()]

  var body: some View {
    let filteredCommands = Binding<[CommandPaletteItem]>(
      get: {
        return searchText.isEmpty
          ? (tree.last!.itemType == .children
            ? tree.last!.children(modelContext: modelContext, appState: appState) : [])
          : (tree.last!.itemType == .children
            ? tree.last!.children(modelContext: modelContext, appState: appState).filter {
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
            if command.onAccept != nil {
              command.onAccept!()
            }
            if command.itemType == .children {
              tree.append(command)
              searchText = ""
            } else {
              if command.id.isNavigationId {
                switch command.id {
                  case .pushCommandPaletteView(let data):
                    commandPaletteNavigation = data
                  default:
                    print("Error: This should never be reached.")
                }
              } else {
                executeCommandPaletteAction(action: command.id)
              }
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
    .navigationDestination(
      item: $commandPaletteNavigation,
      destination: { nav in
        switch nav {
          case .searchByTag(let tag):
            SearchByTagView(item: tag)
          case .searchByTopic(let topic):
            SearchByTopicView(item: topic)
          case .searchBySubject(let subject):
            SearchBySubjectView(item: subject)
        }
      }
    )
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
        if results.isEmpty {
          VStack {
            Text(tree.last?.noneFoundText ?? "No results found")
              .font(.headline)
              .padding()
          }
        } else {
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
      }
      .background(colorScheme == .dark ? .black : .white)
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
