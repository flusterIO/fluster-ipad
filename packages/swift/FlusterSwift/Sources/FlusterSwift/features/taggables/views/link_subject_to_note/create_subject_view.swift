//
//  File.swift
//  FlusterSwift
//
//  Created by Andrew on 12/4/25.
//

import SwiftData
import SwiftUI

public struct CreateSubjectView: View {
  @State private var textValue: String = ""
  @Environment(\.dismiss) private var dismiss
  @Environment(\.modelContext) private var modelContext
  @Binding var selectedSubject: SubjectModel?
  @Binding var paths: [CreateNotePath]
  public init(selectedSubject: Binding<SubjectModel?>, paths: Binding<[CreateNotePath]>) {
    self._selectedSubject = selectedSubject
    self._paths = paths
  }
  public var body: some View {
    Form {
      TextField(
        text: $textValue,
        label: {
          Label("Create", systemImage: "plus")
        }
      )
      HStack {
        Spacer()
        Button(
          action: {
            if !textValue.isTrimmedEmpty() {
              let fetchDescriptor = FetchDescriptor<
                SubjectModel
              >()
              do {
                let subjects = try modelContext.fetch(
                  fetchDescriptor
                )
                let existingSubject = subjects.first(where: { sub in
                  sub.value == textValue
                })
                selectedSubject =
                  existingSubject
                  ?? SubjectModel(value: textValue)
                if existingSubject == nil {
                  print("Creating new subject")
                  modelContext.insert(selectedSubject!)
                }
                textValue = ""
                paths = []
              } catch {
                print(
                  "A error occurre while saving this subject: \(error)"
                )
              }
            }
          },
          label: {
            Text("Create")
          }
        )
        .disabled(textValue.isEmpty)
      }
    }
    .navigationTitle("Create subject")
  }
}

#Preview {
  CreateSubjectView(selectedSubject: .constant(nil), paths: .constant([]))
}
