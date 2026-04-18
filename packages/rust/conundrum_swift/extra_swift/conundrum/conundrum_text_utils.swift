//
//  File.swift
//  FlusterSwift
//
//  Created by Andrew on 3/31/26.
//
import Foundation

public struct ConundrumTextUtils {
  public init() {
  }
    public static func getTitleGroup(content: String, modifiers: [ConundrumModifier], target: ConundrumCompileTarget) async -> TitleGroup? {
    do {
      let res = try await getTitle(
        content: content, modifiers: modifiers, target: target)
      return res
    } catch {
      print("Error retrieving note title: \(error.localizedDescription)")
    }
    return nil
  }
    public static func getTitleGroupSync(content: String, modifiers: [ConundrumModifier], target: ConundrumCompileTarget) -> TitleGroup? {
    do {
      let res = try getTitleSync(
        content: content, modifiers: modifiers, target: target)
      return res
    } catch {
      print("Error retrieving note title: \(error.localizedDescription)")
    }
    return nil
  }
}
