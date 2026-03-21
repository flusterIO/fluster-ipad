//
//  File.swift
//  FlusterAI
//
//  Created by Andrew on 3/21/26.
//

import Foundation
import FoundationModels
import Playgrounds

#Playground {
    let model = SystemLanguageModel.default
    let avail = model.availability
    let session = LanguageModelSession()
    let prompt = "Flatter me, I've been having a bad day."
    Task {
        let res = try await session.respond(to: prompt)
        print(res.content)
    }
}
