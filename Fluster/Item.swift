//
//  Item.swift
//  Fluster
//
//  Created by Andrew on 10/28/25.
//

import Foundation
import SwiftData

@Model
final class Item {
    var timestamp: Date
    
    init(timestamp: Date) {
        self.timestamp = timestamp
    }
}
