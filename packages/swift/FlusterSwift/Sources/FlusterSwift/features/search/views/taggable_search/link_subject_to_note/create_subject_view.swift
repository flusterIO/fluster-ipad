//
//  File.swift
//  FlusterSwift
//
//  Created by Andrew on 12/4/25.
//

import SwiftUI

struct CreateTagView: View {
    @State private var textValue: String = ""
    @Environment(\.dismiss) private var dismiss
    @Binding var selectedSubject: SubjectModel?
    var body: some View {
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
                        if !textValue.isEmpty {
                            selectedSubject = SubjectModel(value: textValue)
                            textValue = ""
                            dismiss()
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
    CreateTagView(selectedSubject: .constant(nil))
}
