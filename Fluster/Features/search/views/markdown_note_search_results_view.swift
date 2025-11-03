//
//  markdown_note_search_result.swift
//  Fluster
//
//  Created by Andrew on 11/2/25.
//

import SwiftUI
import SwiftData

struct MarkdownNotesSearchResultsView: View {
    @Environment(\.modelContext) var modelContext
    @Query var notes: [MarkdownNote]
    
    var body: some View {
        HStack{
            
        }
    }
}

#Preview {
    MarkdownNotesSearchResultsView()
}
