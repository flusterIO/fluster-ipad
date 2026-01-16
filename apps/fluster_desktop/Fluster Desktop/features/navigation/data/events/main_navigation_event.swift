//
//  main_navigation_event.swift
//  Fluster Desktop
//
//  Created by Andrew on 1/15/26.
//

import Foundation
import Combine

class MainNavigationEventEmitter {
    static let shared = MainNavigationEventEmitter()
    let viewUpdatePublisher = PassthroughSubject<MainViewKey, Never>()
    
    func emitChange(to newValue: MainViewKey) {
        viewUpdatePublisher.send(newValue)
    }
}
