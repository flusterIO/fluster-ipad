//
//  bibliography_setting_section_view.swift
//  Fluster
//
//  Created by Andrew on 2/21/26.
//

import SwiftUI
import FlusterData

struct BibliographySettingSectionView: View {
    @AppStorage(AppStorageKeys.embeddedCslFile.rawValue) private var embeddedCslFile: EmbeddedCslFileSwift = .apa
    var body: some View {
        SettingsSection(title: "Bibliography", content: {
            VStack(alignment: .leading) {
                Picker(selection: $embeddedCslFile, content: {
                    ForEach(EmbeddedCslFileSwift.allCases, id: \.rawValue) { item in
                        Text(item.rawValue)
                    }
                }, label: {
                    Text("Citation Format")
                })
                .padding()
            }
            .frame(maxWidth: .infinity)
        })
    }
}

#Preview {
    BibliographySettingSectionView()
}
