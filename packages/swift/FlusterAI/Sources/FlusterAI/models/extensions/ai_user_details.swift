//
//  File.swift
//  FlusterAI
//
//  Created by Andrew on 3/27/26.
//

import Foundation
import FlusterData


extension AIUserDetails {
    func toUserDescription() -> String {
       return """
           The current user prefers to go by \(self.preferred_name).
           """
    }
}
