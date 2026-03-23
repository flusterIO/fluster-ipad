//
//  File.swift
//  FlusterSwift
//
//  Created by Andrew on 3/23/26.
//

import SwiftUI
import FlusterData


public struct UserNameSettingInput: View {
    @AppStorage(AppStorageKeys.userPreferredName.rawValue) private var userName: String = ""
    public init() {
    }
    public var body: some View {
        TextField(text: $userName, prompt: Text("Dr. Albert Einstein"), label: {
            Label(title: {
                Text("Name")
            }, icon: {
                Image(systemName: "person")
            })
        })
    }
}

#Preview {
    UserNameSettingInput()
}
