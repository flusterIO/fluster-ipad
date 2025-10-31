//
//  side_menu_row_view.swift
//  Fluster
//
//  Created by Andrew on 10/31/25.
//

import SwiftUI


struct SideMenuRowView: View {
    var item: NavigationItem
    @Environment(ThemeManager.self) private var themeManager: ThemeManager

    var body: some View {
        HStack {
            Image(systemName: item.icon)
                .imageScale(.small)
                .foregroundStyle(themeManager.theme.popover_foreground)
            Text(item.label)
                .font(.subheadline)
                .foregroundStyle(themeManager.theme.popover_foreground)
            Spacer()
        }
        .padding(.leading)
        .frame(height: 48)
    }
}

#Preview {
    SideMenuRowView(item: navigationItems[0])
        .environment(ThemeManager(initialTheme: FlusterDark()))
}
