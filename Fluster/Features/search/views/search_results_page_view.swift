//
//  search_results_page_view.swift
//  Fluster
//
//  Created by Andrew on 10/29/25.
//

import SwiftUI

struct SearchResultsPageView: View {
    @State private var inputValue: String = ""
    var body: some View {
        VStack {
            TextField("Search", text: $inputValue)
            Text("Search Results")
        }
    }
}

#Preview {
    SearchResultsPageView()
}
