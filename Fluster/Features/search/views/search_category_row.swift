//
//  search_category_row.swift
//  Fluster
//
//  Created by Andrew on 11/1/25.
//

import SwiftUI

struct SearchCategoryRow: View {
    var item: SearchCategoryItem
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
    SearchCategoryRow(item: searchCategoryItems[0])
        .environment(ThemeManager(initialTheme: FlusterDark()))
}
