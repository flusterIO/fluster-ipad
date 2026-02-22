//
//  paper_tab_view.swift
//  Fluster
//
//  Created by Andrew on 2/22/26.
//

import SwiftUI
import FlusterData
import PencilKit
import FlusterSwift

struct PaperTabView: View {
    @State private var toolbar: PKToolPicker = PKToolPicker()
    @Environment(ThemeManager.self) private var themeManager: ThemeManager
    @AppStorage(AppStorageKeys.hasLaunchedPreviously.rawValue) private
      var hasPreviouslyLaunched: Bool = false
    @Binding var editingNote: NoteModel?
    @Binding var selectedTab: IpadMainViewTab
    init(
      editingNote: Binding<NoteModel?>,
      selectedTab: Binding<IpadMainViewTab>
    ) {
      self._editingNote = editingNote
      self._selectedTab = selectedTab
    }
    var body: some View {
        if let en = Binding($editingNote)  {
            PaperKitView(editingNote: en)
        } else {
           NoNotesFoundView()
        }
    }
}
