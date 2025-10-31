//
//  sice_menu_view.swift
//  Fluster
//
//  Created by Andrew on 10/31/25.
//

import SwiftUI

struct SideMenuView: View {
    @Binding var open: Bool
    @Environment(ThemeManager.self) private var themeManager: ThemeManager
    @Environment(Coordinator<MainCoordinatorPages>.self) private var coordinator

    var body: some View {
        ZStack {
            if open {
                HStack {
                    VStack(alignment: .leading, spacing: 32) {
                        SideMenuHeaderView()
                        VStack {
                            ForEach(navigationItems) { item in
                                Button(
                                    action: {
                                        if item.id != "home" {
                                            coordinator.push(item.path)
                                        } else {
                                            open = false
                                        }
                                    },
                                    label: {
                                        SideMenuRowView(item: item)
                                    }
                                )
                            }
                        }
                        Spacer()
                    }
                    .padding()
                    .frame(width: 270, alignment: .leading)
                    .background(themeManager.theme.popover)

                    Spacer()
                }
            }
        }

        .transition(.move(edge: .leading))
        .animation(.easeInOut, value: open)
    }
}
