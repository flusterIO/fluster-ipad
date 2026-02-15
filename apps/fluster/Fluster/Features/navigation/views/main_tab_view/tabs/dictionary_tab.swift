//
//  dictionary_tab.swift
//  Fluster
//
//  Created by Andrew on 12/25/25.
//

import SwiftUI
import FlusterSwift

struct DictionaryTab: View {
    let container: DictionaryWebviewContainer = DictionaryWebviewContainer(bounce: false, scrollEnabled: false, onLoad: nil)
    var body: some View {
        DictionaryWebview(container: container)
    }
}

#Preview {
    DictionaryTab()
}
