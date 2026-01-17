//
//  auto_setting_table_footer_view.swift
//  Fluster
//
//  Created by Andrew on 1/17/26.
//

import FlusterData
import SwiftUI

struct AutoSettingTableFooterView: View {
  @Binding private var sortOrder: [KeyPathComparator<AutoTaggable>]
  @Binding private var pageIdx: Int
  @Binding private var count: Double
  let perPage: Int = 5
  public init(
    sortOrder: Binding<[KeyPathComparator<AutoTaggable>]>, pageIdx: Binding<Int>,
    count: Binding<Double>
  ) {
    self._sortOrder = sortOrder
    self._pageIdx = pageIdx
    self._count = count
  }
  var body: some View {
    HStack {
      Text("Page \(pageIdx + 1) of \(Int(ceil(count / (Double(pageIdx + 1) * Double(perPage)))))")
        .font(.caption)
    }
  }
}

#Preview {
  AutoSettingTableFooterView(
    sortOrder: .constant([KeyPathComparator(\AutoTaggable.value)]
    ), pageIdx: .constant(0),
    count: .constant(10)
  )
}
