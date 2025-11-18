//
//  by_bib_entry_search_result_view.swift
//  Fluster
//
//  Created by Andrew on 11/10/25.
//

import SwiftUI

struct ByBibEntrySearchResults: View {
    var bibEntryId: String
    @Environment(\.presentationMode) var presentationMode: Binding<PresentationMode>
    
    var body: some View {
        VStack {
           Text("Here mothafucka...")
        }
        .toolbar {
            ToolbarItem(placement: .topBarLeading, content: {
                Button(action: {
                    print("Back mothafucka...")
                    self.presentationMode.wrappedValue.dismiss()
                }, label: {
                    Label("Back", image: "plus")
                })
            })
        }
    }
}
