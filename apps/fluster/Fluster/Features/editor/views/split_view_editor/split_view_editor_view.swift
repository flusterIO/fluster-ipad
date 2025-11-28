//
//  split_view_editor_view.swift
//  Fluster
//
//  Created by Andrew on 11/28/25.
//

import SwiftUI
import SplitView

struct SplitViewEditorView: View {
    var body: some View {
        HSplit(left: {
            SplitViewEditorOnlyView()
        }, right: {
            SplitViewPreviewOnlyView()
        })
    }
}

#Preview {
    SplitViewEditorView()
}
