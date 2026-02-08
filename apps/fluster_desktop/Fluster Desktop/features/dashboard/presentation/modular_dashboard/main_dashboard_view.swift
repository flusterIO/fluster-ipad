//
//  main_dashboard_view.swift
//  fluster_desktop
//
//  Created by Andrew on 1/14/26.
//

import SwiftUI

struct ModularDashboardView: View {
  var body: some View {
    GeometryReader { geo in
      ScrollView {
        VStack(alignment: .center) {
          Grid(alignment: .center) {
            QuickActionDashboardSectionView(
              columns: geo.size.width >= 768 ? .wider : geo.size.width >= 640 ? .wide : .narrow
            )
            .padding(.horizontal)
            if geo.size.width >= 768 {
              GridRow {
                RecentNotesDashboardSectionView()
                  .padding(.leading)
                VStack {
                  TopicsDashboardSectionView()
                    .frame(maxWidth: 350)
                  SubjectsDashboardSectionView()
                    .frame(maxWidth: 350)
                }
                .padding(.trailing)
              }
            } else {
              RecentNotesDashboardSectionView()
                .padding(.horizontal)
              GridRow {
                TopicsDashboardSectionView()
                SubjectsDashboardSectionView()
              }
              .padding(.horizontal)
            }
          }
          .frame(maxWidth: 1080)
        }
        .frame(maxWidth: .infinity)
      }
    }
  }
}

#Preview {
  ModularDashboardView()
}
