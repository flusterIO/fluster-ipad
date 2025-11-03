//
//  md_note.swift
//  Fluster
//
//  Created by Andrew on 10/29/25.
//

import Foundation
import SwiftData

func removeHeadingSyntax(val: String) -> String {
    if val.hasPrefix("#") {
        return removeHeadingSyntax(val: String(val.suffix(val.count - 1)))
    }
    return val.trimmingCharacters(in: NSCharacterSet.whitespacesAndNewlines)
}

func getTitle(body: String) -> String? {
    let lines = body.components(separatedBy: .newlines)
    let firstTitleLine = lines.first(where: { $0.hasPrefix("#") })
    if firstTitleLine == nil {
        return nil
    }
    return removeHeadingSyntax(val: firstTitleLine!)
}

@Model
class MarkdownNote {
    var body: String
    var title: String?
    
    init(body: String) {
        self.body = body
        self.title = getTitle(body: body)
    }
    
    
}
