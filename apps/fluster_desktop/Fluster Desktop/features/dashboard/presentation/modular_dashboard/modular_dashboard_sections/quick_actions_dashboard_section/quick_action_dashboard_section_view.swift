//
//  quick_action_dashboard_section_view.swift
//  Fluster
//
//  Created by Andrew on 1/18/26.
//

import SwiftUI

public enum QuickActionColumns {
  case narrow, wide, wider
}

struct QuickActionDashboardSectionView: View {
  let columns: QuickActionColumns
  var body: some View {
    VStack(alignment: .center) {
      VStack(alignment: .leading, spacing: 8) {
        VStack(alignment: .leading, spacing: 4) {
          HStack(alignment: .center) {
            Image(systemName: "plus")
              .font(.title)
            Text("Quick Actions")
              .lineLimit(1)
              .font(.title)
          }
          Text("Quick and frequently used actions")
            .lineLimit(1)
            .font(.subheadline)
            .foregroundStyle(.secondary)
        }
//        .frame(maxWidth: .infinity)
        Grid {
          if columns == .wider {
            GridRow {
              ForEach(quickActions, id: \.label) { qa in
                QuickActionItemView(
                  item: qa,
                )
              }
            }
          } else if columns == .wide {
            GridRow {
              ForEach(quickActions[0..<3], id: \.label) { qa in
                QuickActionItemView(
                  item: qa
                )
              }
            }
            GridRow {
              ForEach(quickActions[3..<quickActions.count], id: \.label) { qa in
                QuickActionItemView(
                  item: qa
                )
              }
            }
          } else {
            ForEach(quickActions, id: \.label) { qa in
              GridRow {
                QuickActionItemView(item: qa)
              }
            }
          }
        }
        .padding(.top, 16)
      }
    }
    .padding()
    .glassEffect(in: .rect(cornerRadius: 12))
  }
}

#Preview {
  QuickActionDashboardSectionView(columns: .wider)
}
