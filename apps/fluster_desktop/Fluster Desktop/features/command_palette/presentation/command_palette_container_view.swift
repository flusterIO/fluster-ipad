//
//  command_palette_container_view.swift
//  Fluster Desktop
//
//  Created by Andrew on 1/14/26.
//

import Combine
import SwiftUI

struct CommandPaletteContainerView: View {
  public var close: () -> Void
  @State private var searchText: String = ""
  @AppStorage(DesktopAppStorageKeys.colorScheme.rawValue) private var selectedTheme: AppTheme =
    .dark
  @Environment(\.modelContext) private var modelContext
  @EnvironmentObject private var appState: AppState

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
      CommandPaletteView(
        searchText: $searchText,
        results: filteredCommands,
        onClose: {
          searchText = ""
          tree = [CommandPaletteRoot()]
          close()
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
                  appState.commandPaletteNavigate(to: data)
                default:
                  print("Error: This should never be reached.")
              }
            } else {
              executeCommandPaletteAction(action: command.id)
            }
            tree = [CommandPaletteRoot()]
            searchText = ""
            close()
          }
        }
      )
      .environment(\.colorScheme, selectedTheme.colorScheme)
      .frame(maxWidth: .infinity, maxHeight: .infinity)
      .transition(.opacity.combined(with: .scale))
      .zIndex(10)
      .onAppear { searchFieldFocused = true }
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
  @AppStorage(DesktopAppStorageKeys.colorScheme.rawValue) private var selectedTheme: AppTheme =
    .dark

  var body: some View {
    ScrollViewReader { proxy in
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
          .frame(maxHeight: .infinity)
        } else {
          ScrollView {
            VStack(alignment: .leading, spacing: 0) {
              ForEach(Array(results.enumerated()), id: \.offset) { idx, command in
                CommandPaletteItemView(
                  command: command, idx: idx, focusedIndex: $focusedIndex,
                  onCommandSelected: onCommandSelected
                )
                .id(command.uniqueId)
              }
            }
          }
          .scrollIndicators(.hidden)
        }
      }
      .background(selectedTheme.colorScheme == .dark ? .black : .white)
      .cornerRadius(18)
      .frame(maxWidth: 768, maxHeight: .infinity)
      .onAppear { searchFieldFocused = true }
      .onExitCommand {
        onClose()
      }
      .onChange(
        of: focusedIndex,
        {
          if results.count > focusedIndex {
            let item = results[focusedIndex]
            proxy.scrollTo(item.uniqueId, anchor: .center)
          }
        })
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
  CommandPaletteContainerView(close: {})
}
