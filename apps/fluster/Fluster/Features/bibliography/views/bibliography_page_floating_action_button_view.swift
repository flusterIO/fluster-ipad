//
//  bibliography_page_floating_action_button_view.swift
//  Fluster
//
//  Created by Andrew on 11/23/25.
//

import FloatingButton
import SwiftUI

struct BibliographyPageFloatingButtonView: View {
    @Binding var editing: BibEntryModel?
    let bibEditorContainer: BibtexEditorWebviewContainer
    @State private var isOpen: Bool = false
    @Environment(ThemeManager.self) private var themeManager: ThemeManager

    var body: some View {
        FloatingButton(
            mainButtonView:
                ZStack {
                    themeManager.theme.primary
                        .clipShape(
                            RoundedRectangle(cornerRadius: .infinity)
                        )
                        .frame(width: 80, height: 80)
                    Image(systemName: "plus")
                        .imageScale(.large)
                        .foregroundStyle(
                            themeManager.theme.primary_foreground
                        )
                        .clipShape(Circle())
                        .padding()
                },
            buttons: [
                NavigationLink(destination: {
                   CreateBibEntrySheetView(editingBibEntry: $editing, container: bibEditorContainer)
                        .onAppear{
                            isOpen = false
                        }
                }, label: {
                    Label("New Entry", systemImage: "plus")
                })
            ],
            isOpen: $isOpen
        )
        .straight()
        .direction(.top)
        .alignment(.right)
        .spacing(10)
        .initialOffset(x: 1000)
        .animation(.spring())
    }
}

#Preview {
    BibliographyPageFloatingButtonView(editing: .constant(nil), bibEditorContainer: BibtexEditorWebviewContainer())
}
