//
//  setting_section_view.swift
//  Fluster Desktop
//
//  Created by Andrew on 1/15/26.
//

import SwiftUI


public struct SettingsSection<Content: View>: View {
    let title: String
    let content: Content
    init(title: String, @ViewBuilder content: () -> Content) {
        self.title = title
        self.content = content()
    }
    public var body: some View {
        VStack(alignment: .leading, spacing: 14) {
            Text(title)
                .font(.headline)
                .foregroundStyle(.secondary)
            VStack(alignment: .leading, spacing: 10) {
                content
            }
            .padding()
            .clipShape(RoundedRectangle(cornerRadius: 0, style: .continuous))
            .glassEffect(in: .rect(cornerRadius: 16))
        }
       .padding(.horizontal)
    }
}
