//
//  File.swift
//  FlusterData
//
//  Created by Andrew on 1/16/26.
//

import SwiftUI

public extension AutoTaggableType {
    func taggableColor() -> Color {
        switch self {
        case .subject:
           return Color.red
        case .topic:
            return Color.green
        case .tag:
            return Color.blue
            
        }
    }
}
