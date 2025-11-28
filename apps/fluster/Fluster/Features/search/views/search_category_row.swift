//
//  search_category_row.swift
//  Fluster
//
//  Created by Andrew on 11/1/25.
//

import SwiftUI
import FlusterSwift

struct SearchCategoryRow: View {
    var item: SearchCategoryItem
    @Binding var activeCategory: SearchCategoryId
    @Environment(ThemeManager.self) private var themeManager: ThemeManager

    var body: some View {
        HStack {
            Image(systemName: item.icon)
                .imageScale(.small)
                .foregroundStyle(themeManager.theme.popover_foreground.opacity(activeCategory == item.id ? 1 : 0.7))
            Text(item.label)
                .font(.subheadline)
                .foregroundStyle(themeManager.theme.popover_foreground.opacity(activeCategory == item.id ? 1 : 0.7))
            Spacer()
        }
        .padding(.leading)
        .frame(height: 48)
    }
}

#Preview {
    SearchCategoryRow(item: searchCategoryItems[0], activeCategory: .constant(.note))
        .environment(ThemeManager(initialTheme: FlusterDark()))
}
