//
//  File.swift
//  FlusterSwift
//
//  Created by Andrew on 3/12/26.
//

import SwiftUI
import FlusterData


#if os(ios)
struct IosWebviewContainer: View {
    let url: URL
    let implementation: WebviewImplementation
    @Binding public var editingNote: NoteModel?
    var body: some View {
        WebviewContainer(onLoad: {
            
        }, editingNote: $editingNote, implementation: implementation)
    }
}
#endif
