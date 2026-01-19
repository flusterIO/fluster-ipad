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
  let perPage: Int
  var maxPage: Int {
    return Int(ceil(count / Double(perPage)))
  }
  public init(
    sortOrder: Binding<[KeyPathComparator<AutoTaggable>]>, pageIdx: Binding<Int>,
    count: Binding<Double>,
    perPage: Int
  ) {
    self._sortOrder = sortOrder
    self._pageIdx = pageIdx
    self._count = count
    self.perPage = perPage
  }

  var body: some View {
    HStack(alignment: .center) {
      Text("Page \(pageIdx + 1) of \(maxPage + 1)")
        .font(.caption)
      Spacer()
      HStack {
        Button(
          action: {
            decrementPageIdx()
          },
          label: {
            Image(systemName: "minus")
              .frame(width: 20, height: 20)
          }
        )
        .controlSize(.large)
        .buttonBorderShape(.capsule)
        .disabled(pageIdx <= 0)
        Button(
          action: {
            incremementPageIdx()
          },
          label: {
            Image(systemName: "plus")
              .frame(width: 20, height: 20)
          }
        )
        .controlSize(.large)
        .buttonBorderShape(.capsule)
        .disabled(pageIdx >= self.maxPage)
      }
    }
    .frame(maxWidth: .infinity)
  }
  func incremementPageIdx() {
    let maxPage = self.maxPage
    if pageIdx < maxPage {
      pageIdx += 1
    } else {
      pageIdx = 0
    }
  }
  func decrementPageIdx() {
    if pageIdx > 0 {
      pageIdx -= 1
    } else {
      pageIdx = self.maxPage
    }
  }
}

#Preview {
  AutoSettingTableFooterView(
    sortOrder: .constant([KeyPathComparator(\AutoTaggable.value)]
    ), pageIdx: .constant(0),
    count: .constant(10),
    perPage: 10
  )
}
