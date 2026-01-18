//
//  note_sidebar_section.swift
//  Fluster
//
//  Created by Andrew on 1/17/26.
//

import SwiftUI

struct CollapsableSidebarSection: View {
  @Binding var open: Bool
  let items: [SidebarItem]
  @Binding var mainView: MainViewKey
  let title: LocalizedStringKey
  var body: some View {
    Section(
      isExpanded: $open,
      content: {
        ForEach(items) { item in
          Button(
            action: {
              mainView = item.id
            },
            label: {
              HStack {
                Image(systemName: item.icon)
                Text(item.label)
              }
              .padding(.vertical, 4)
              .frame(maxWidth: .infinity, alignment: .leading)
            }
          )
          .labelStyle(.titleAndIcon)
          .buttonStyle(.bordered)
          .frame(maxWidth: .infinity)
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
    mainView: .constant(.dashboard),
    title: "Test"
  )
}
