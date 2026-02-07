//
//  view_paper_page_view.swift
//  Fluster
//
//  Created by Andrew on 1/20/26.
//

import SwiftUI

struct ViewPaperPageView: View {
    @EnvironmentObject private var appState: AppState
    var body: some View {
        if appState.editingNote != nil {
            Text("View Paper")
        } else {
            NoNoteSelectedView()
        }
    }
}

#Preview {
    ViewPaperPageView()
}
