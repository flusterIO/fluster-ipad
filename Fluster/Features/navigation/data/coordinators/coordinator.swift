//
//  coordinator.swift
//  Fluster
//
//  Created by Andrew on 10/29/25.
//

import SwiftUI

@Observable
class Coordinator<CoordinatorPage: Coordinatable> {

    var path: NavigationPath = NavigationPath()
    var sheet: CoordinatorPage?
    var fullScreenCover: CoordinatorPage?

    enum PushType {
        case link
        case sheet
        case fullScreenCover
    }

    enum PopType {
        case link(last: Int)
        case sheet
        case fullScreenCover
    }

    func push(_ page: CoordinatorPage, type: PushType = .link) {
        switch type {
        case .link:
            path.append(page)
        case .sheet:
            sheet = page
        case .fullScreenCover:
            fullScreenCover = page
        }
    }
    
    func pop(type: PopType = .link(last: 1)) {
        switch type {
        case .link(let last):
            path.removeLast(last)
        case .sheet:
            sheet = nil
        case .fullScreenCover:
            fullScreenCover = nil
        }
    }
    
    func popToRoot() {
        path.removeLast(path.count)
    }

}
