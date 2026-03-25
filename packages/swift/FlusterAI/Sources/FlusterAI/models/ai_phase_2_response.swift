//
//  File.swift
//  FlusterAI
//
//  Created by Andrew on 3/24/26.
//

import Foundation

public struct AiPhase2Response {
  public let success: Bool
  /// The id is likely going to be passed in to the component so that the AI generated data can be received by the same component that sent the request.
  public let id: String
  public let model: String
  public let replaceWith: String?
  public let userMessage: String?
  public init(success: Bool, replaceWith: String?, userMessage: String?, id: String?, model: String)
  {
    self.success = success
    self.replaceWith = replaceWith
    self.userMessage = userMessage
    self.model = model
    self.id = id ?? UUID().uuidString
  }
}
