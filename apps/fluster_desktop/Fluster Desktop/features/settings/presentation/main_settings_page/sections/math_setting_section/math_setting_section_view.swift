//
//  math_setting_section_view.swift
//  Fluster
//
//  Created by Andrew on 4/30/26.
//

import SwiftUI
import FlusterSwift

struct MathSettingSectionView: View {
    var body: some View {
        SettingsSection(title: "Math", content: {
            ShowEquationLabelsToggle()
        })
    }
}

#Preview {
    MathSettingSectionView()
}
