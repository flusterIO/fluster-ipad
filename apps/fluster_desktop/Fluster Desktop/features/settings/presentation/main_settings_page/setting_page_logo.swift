//
//  setting_page_logo.swift
//  Fluster
//
//  Created by Andrew on 2/8/26.
//

import FlusterSwift
import SwiftUI

struct SettingPageLogo: View {
  init() {
      // WARNING: This is a huge no-no. Make sure to remove this after seeding debugging is done.
    UserDefaults.resetUserDefaultsDEVELOPMENTONLY()
  }
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
