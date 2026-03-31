//
//  File.swift
//  FlusterAI
//
//  Created by Andrew on 3/22/26.
//

import FlusterData
import Foundation
import FoundationModels

public extension SystemLanguageModel.Availability {
  public func toReduxRepresentation() -> FoundationModelAccessStatus {
    switch self {
      case .available:
        return .available
      case .unavailable(let reason):
        switch reason {
          case .appleIntelligenceNotEnabled:
            return .appleIntelligenceNotEnabled
          case .deviceNotEligible:
            return .deviceNotEligible
          case .modelNotReady:
            return .modelNotReady
          default:
            return .unknownStatus
        }
      default:
        return .unknownStatus
    }
  }
}
