//
//  coordinator_stack.swift
//  Fluster
//
//  Created by Andrew on 10/29/25.
//

import SwiftUI

struct CoordinatorStack<CoordinatorPage: Coordinatable>: View {
    
    let root: CoordinatorPage
    
    init(_ root: CoordinatorPage) {
        self.root = root
    }
    
    @State private var coordinator = Coordinator<CoordinatorPage>()
    @State private var themeManager = ThemeManager(initialTheme: FlusterDark())
    
    var body: some View {
        NavigationStack(path: $coordinator.path) {
            root
                .navigationDestination(for: CoordinatorPage.self) { $0 }
                .sheet(item: $coordinator.sheet) { $0 }
                .fullScreenCover(item: $coordinator.fullScreenCover) { $0 }
        }
        .environment(themeManager)
        .environment(coordinator)
    }
}

#Preview {
    CoordinatorStack(MainCoordinatorPages.root)
}
