//
//  create_note_tip.swift
//  Fluster
//
//  Created by Andrew on 5/31/26.
//

import Foundation
import TipKit


public struct CreateNoteTip: Tip {
    public init() {}
    public var options: [Option] {
        MaxDisplayCount(1)
    }
    public var title: Text {
        Text("Create your first note by clicking here!")
    }
    public var message: Text? {
        Text("See the documentation at flusterapp.com for more information")
    }
    public var image: Image? {
        Image(systemName: "document.badge.plus")
    }
}
