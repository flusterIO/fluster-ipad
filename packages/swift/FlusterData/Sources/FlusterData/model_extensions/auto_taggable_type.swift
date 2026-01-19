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
           return Color.indigo
        case .topic:
            return Color.orange
        case .tag:
            return Color.blue
            
        }
    }
}
