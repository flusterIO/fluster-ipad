//
//  File.swift
//  FlusterSwift
//
//  Created by Andrew on 12/11/25.
//

import SwiftData
import SwiftUI

// BETA: It looks decent but there's a bug in the button that causes the menu to open. Fix this up at some point, but I'm setting this aside for now.
struct SubjectInputView: View {
  @Query private var subjects: [SubjectModel]
  @State private var selectedSubject: SubjectModel?
  @State private var sheetTextValue: String = ""
  @State private var showSheet: Bool = false
  @Environment(ThemeManager.self) private var themeManager: ThemeManager
  @Binding var editingNote: NoteModel
  var body: some View {
    HStack(spacing: 16) {
      Picker("Subject", selection: $selectedSubject) {
        ForEach(subjects, id: \.id) { subject in
          Text(subject.value).tag(subject)
        }
      }
      .padding()
      .roundedCornerWithBorder(lineWidth: 2, borderColor: themeManager.theme.border, radius: 16)
      .padding()
      Button(
        action: {
          print("Here")
        },
        label: {
          Label("Create", systemImage: "plus")
        }
      )
    }
    .sheet(
      isPresented: $showSheet,
      onDismiss: {
        if !sheetTextValue.isEmpty {
          self.selectedSubject = SubjectModel(value: sheetTextValue)
        }
      },
      content: {
        Form {
          TextField(
            text: $sheetTextValue,
            label: {
              Text("Subject")
            })
        }
      })
  }
}
