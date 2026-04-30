//
//  File.swift
//  FlusterSwift
//
//  Created by Andrew on 4/29/26.
//

import ConundrumSwift
import FlusterData
import SwiftUI

public struct ShowEquationLabelsToggle: View {
  @AppStorage(AppStorageKeys.showEquationLabels.rawValue) private var strategy:
    EquationNumberingStrategy =
      .all
  public init() {
  }
  public var body: some View {
    VStack(alignment: .leading, spacing: 16) {
      Text("Equation Labels")
        .font(.headline)
      Picker(
        selection: $strategy,
        content: {
          ForEach(EquationNumberingStrategy.allCases, id: \.self) { strat in
            Text(strat.toString()).tag(strat)
          }
        },
        label: {
          Label(
            title: {
              Text("Equation Labels")
            },
            icon: {
              Image(systemName: "function")
            })
        }
      )
      .labelsHidden()
      .pickerStyle(.inline)
    }.frame(alignment: .leading)
  }
}

#Preview {
  ShowEquationLabelsToggle()
}
