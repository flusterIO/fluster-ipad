//
//  File.swift
//  FlusterAI
//
//  Created by Andrew on 3/21/26.
//

import Foundation
import FoundationModels
import Playgrounds

@Generable(description: "A description of the city based on your best analysis.")
struct CityDescription {
    @Guide(description: "The location of the city in a descriptive, easily parsable format.")
    var location: String
    @Guide(description: "A short description of the city in less than 250 words.")
    var desc: String
    @Guide(description: "A rating of the city from 1 to 10, with 10 being one of the best in the world.")
    var rating: Float
}

#Playground {
    let model = SystemLanguageModel.default
    let avail = model.availability
    let session = LanguageModelSession()
    let prompt = "Oak Creek, Wisconsin"
    Task {
        let res = try await session.respond(to: prompt, generating: CityDescription.self)
    }
}
