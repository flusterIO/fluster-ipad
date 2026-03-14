//
//  File.swift
//  FlusterWebviewClients
//
//  Created by Andrew on 3/14/26.
//

import Foundation


public enum EditorErrorMessageId {
    case attemptToSaveEmptyBibEntry, attemptToUpdateOneBibEntryAsMany
}

public struct EditorErrorMessage: Identifiable {
    public let id: EditorErrorMessageId
    public let title: String
    public let desc: String?
    public init(id: EditorErrorMessageId, title: String, desc: String?) {
        self.id = id
        self.title = title
        self.desc = desc
    }
}
