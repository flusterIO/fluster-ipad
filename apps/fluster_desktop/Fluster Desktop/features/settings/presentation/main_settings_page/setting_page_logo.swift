//
//  setting_page_logo.swift
//  Fluster
//
//  Created by Andrew on 2/8/26.
//

import SwiftUI

struct SettingPageLogo: View {
  var body: some View {
    VStack(alignment: .leading, spacing: 24) {
      // Header
      HStack(spacing: 12) {
        Image("flusterIcon")
          .resizable()
          .foregroundStyle(.tint)
          .aspectRatio(contentMode: .fit)
          .frame(width: 48, height: 48, alignment: .center)
        VStack(alignment: .leading) {
          Text("Fluster")
            .font(.title)
            .fontWeight(.bold)
          Text("Settings")
            .font(.headline)
            .foregroundStyle(.secondary)
        }
        Spacer()
      }
      .padding(.horizontal)
      .frame(maxWidth: .infinity)
    }
  }
}

#Preview {
  SettingPageLogo()
}
