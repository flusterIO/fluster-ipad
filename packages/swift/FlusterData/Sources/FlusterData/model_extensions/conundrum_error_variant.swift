//
//  File.swift
//  FlusterData
//
//  Created by Andrew on 4/5/26.
//

import Foundation
import ConundrumSwift

public extension ConundrumErrorVariant {
    public func toConundrumError() -> ConundrumError? {
        switch self {
        case .FailToFindComponent(_):
            return nil
        case .InternalParserError(let e):
            return e
        case .UserFacingGeneralParserError(let e):
            return e
        default:
            return nil
        }
    }
}
