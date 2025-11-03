//
//  create_bib_entry_sheet_view.swift
//  Fluster
//
//  Created by Andrew on 11/3/25.
//

import SwiftUI
import SwiftData

struct CreateBibEntrySheetView: View {
    @State private var inputValue: String = ""
    @Binding var isPresented: Bool
    @Environment(\.modelContext) var modelContext
    @Environment(ThemeManager.self) private var themeManager: ThemeManager

    var body: some View {
        VStack {
            TextEditor(text: $inputValue)
                .padding()
                .border(themeManager.theme.border)
                .frame(minHeight: 150)

            Spacer(minLength: 8)
            VStack(alignment: .trailing) {
                Button("Create") {
                    if inputValue.isEmpty {
                       return
                    }
                    isPresented = false
                    modelContext.insert(BibEntryModel(data: inputValue))
                }
            }
            .padding()
            .buttonStyle(.glassProminent)
            Spacer(minLength: 8)
        }
    }
}

#Preview {
    CreateBibEntrySheetView(isPresented: .constant(true))
        .environment(ThemeManager(initialTheme: FlusterDark()))

}
