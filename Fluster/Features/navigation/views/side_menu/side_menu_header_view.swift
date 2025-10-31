//
//  side_menu_header_view.swift
//  Fluster
//
//  Created by Andrew on 10/31/25.
//

import SwiftUI


struct SideMenuHeaderView: View {
    @Environment(ThemeManager.self) private var themeManager: ThemeManager

    var body: some View {
        HStack {
            Image(systemName: "person.circle.fill")
                .imageScale(.large)
                .foregroundStyle(themeManager.theme.primary_foreground)
                .frame(width: 48, height: 48)
                .background(themeManager.theme.primary)
                .clipShape(RoundedRectangle(cornerRadius: 10))
                .padding(.vertical)
            VStack(alignment: .leading, spacing: 6) {
           Text("Andrew Mueller")
                    .font(.subheadline)
                    .foregroundStyle(themeManager.theme.popover_foreground)
                Text("test@gmail.com")
                    .font(.footnote)
                    .foregroundStyle(themeManager.theme.popover_foreground)
                    .tint(.gray)
                   
            }
        }
        .padding(.top)
    }
}

#Preview {
    SideMenuHeaderView()
        .environment(ThemeManager(initialTheme: FlusterDark()))
}
