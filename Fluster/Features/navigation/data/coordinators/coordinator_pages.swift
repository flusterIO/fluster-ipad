//
//  coordinator_pages.swift
//  Fluster
//
//  Created by Andrew on 10/29/25.
//

import SwiftUI


enum MainCoordinatorPages: Coordinatable {
    var id: UUID { .init() }

    case root

    case search

    case bibliography

    var body: some View {
        switch self {
        case .root:
            MainView()
                .ignoresSafeArea()
        case .bibliography:
            BibliographyPageView()
        case .search:
            SearchResultsPageView()
        }
    }

}
