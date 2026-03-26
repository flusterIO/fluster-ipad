//
//  File.swift
//  FlusterSwift
//
//  Created by Andrew on 3/26/26.
//

import SwiftUI
import FlusterData


public struct StorePlainTextSettingInput: View {
    @AppStorage(AppStorageKeys.storePlainText.rawValue) private var storePlainText: Bool = true
    public init() {
    }
    public var body: some View {
        Toggle(isOn: $storePlainText, label: {
            Label(title: {
                Text("Store Plain Text")
            }, icon: {
                Image(systemName: "swiftdata")
            })
        })
    }
}

#Preview {
    StorePlainTextSettingInput()
}
