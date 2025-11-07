//
//  create_bib_entry_sheet_view.swift
//  Fluster
//
//  Created by Andrew on 11/3/25.
//

import SwiftUI
import SwiftData

struct CreateBibEntrySheetView: View {
    @Binding var inputValue: String
    @Binding var isPresented: Bool
    @Binding var editing: BibEntryModel?
    @Environment(\.modelContext) var modelContext
    @Environment(ThemeManager.self) private var themeManager: ThemeManager

    var body: some View {
        VStack {
            Spacer(minLength: 8)
            HStack(alignment: .center){
                Text("Paste supported bibtex entry.")
                    .font(.caption)
            }
            TextEditor(text: $inputValue)
                .padding()
                .frame(minHeight: 150)

            Spacer(minLength: 8)
            VStack(alignment: .trailing) {
                Button("Create") {
                    isPresented = false
                    let item = BibEntryModel(data: inputValue)
                    if editing != nil {
                        item.id = editing!.id
                    }
                    modelContext.insert(item)
                }
            }
            .padding()
            .buttonStyle(.glassProminent)
            Spacer(minLength: 8)
        }
    }
}

#Preview {
    CreateBibEntrySheetView(inputValue: .constant(""), isPresented: .constant(true), editing: .constant(nil))
        .environment(ThemeManager(initialTheme: FlusterDark()))

}
