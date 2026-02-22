//
//  note_sidebar_section.swift
//  Fluster
//
//  Created by Andrew on 1/17/26.
//

import FlusterData
import SwiftUI

struct CollapsableSidebarSection: View {
  @Binding var open: Bool
  let items: [SidebarItem]
  let title: LocalizedStringKey
  @EnvironmentObject private var appState: AppState
  @Environment(\.dismiss) private var dismiss
  @Environment(\.isPresented) private var isPresented

  var body: some View {
    Section(
      content: {
        if open {
          ForEach(items) { item in
            Button(
              action: {
                appState.mainView = item.id
                if !appState.commandPaletteNavigation.isEmpty {
                  appState.commandPaletteNavigation.removeLast(
                    appState.commandPaletteNavigation.count)
                }
              },
              label: {
                HStack {
                  Image(systemName: item.icon.toSfIcon())
                  Text(item.label)
                }
                .padding(.vertical, 4)
                .frame(maxWidth: .infinity, alignment: .leading)
              }
            )
            .labelStyle(.titleAndIcon)
            .buttonStyle(.accessoryBarAction)
            .foregroundStyle(
              appState.mainView == item.id
                ? Color.white : Color.primary
            )
            .background(
              appState.mainView == item.id
                ? RoundedRectangle(cornerRadius: 8).fill(Color.blue)
                : RoundedRectangle(cornerRadius: 8).fill(Color.clear)
            )
            .frame(maxWidth: .infinity)
          }
        }
      },
      header: {
        Button(
          action: {
            withAnimation {
              open.toggle()
            }
          },
          label: {
            HStack {
              Image(systemName: "chevron.down")
                .rotation3DEffect(Angle.degrees(open ? 0 : 180), axis: (1, 0, 0))
              Text(title)
                .font(.headline)
                .frame(maxWidth: .infinity, alignment: .leading)
            }
          }
        )
        .padding(.top, 12)
        .buttonStyle(.plain)
      }
    )
    .padding(.horizontal)
  }
}

#Preview {
  CollapsableSidebarSection(
    open: .constant(true),
    items: noteSideBarItems,
    title: "Test"
  )
}
