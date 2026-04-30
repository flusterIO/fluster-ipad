//
//  File.swift
//  FlusterData
//
//  Created by Andrew on 4/30/26.
//

import ConundrumSwift
import Foundation

extension EquationNumberingStrategy {
  public func toString() -> String {
    switch self {
      case .all:
        "All"
      case .idOnly:
        "Id'd only"
      case .none:
        "None"
    }
  }
}
