//
//  File.swift
//  FlusterSwift
//
//  Created by Andrew on 4/16/26.
//

import Foundation


public struct NotificationItem: Identifiable {
    public var id: UUID
    public var msg: String
    public  static func fromMessage(_ msg: String) -> Self {
        NotificationItem(id: UUID.init(), msg: msg)
    }
}


