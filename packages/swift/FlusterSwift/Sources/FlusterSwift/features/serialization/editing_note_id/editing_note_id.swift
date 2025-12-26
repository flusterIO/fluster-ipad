//
//  File.swift
//  FlusterSwift
//
//  Created by Andrew on 12/17/25.
//

import Foundation
import SwiftData

public class EditingNoteId: Codable {
  public static let editingNoteIdProtocolLeader: String = "userDefinedId://"
  public let value: String
  public let isUserDefined: Bool
  public let id: String

  public init(value: String) {
    self.value = value
    if value.starts(with: EditingNoteId.editingNoteIdProtocolLeader) {
      self.isUserDefined = true
      self.id = self.value.replacingOccurrences(
        of: EditingNoteId.editingNoteIdProtocolLeader, with: "")
    } else {
      self.isUserDefined = false
      self.id = self.value
    }
  }

  public static func fromUserDefinedId(userDefinedId: String) -> EditingNoteId {
    EditingNoteId(value: "\(EditingNoteId.editingNoteIdProtocolLeader)\(userDefinedId)")
  }

  nonisolated(nonsending)
    public func getEditingNoteId(context: ModelContext) async -> NoteModel?
  {
    let _id = self.id
    let fetchDescriptor =
      self.isUserDefined
      ? FetchDescriptor<NoteModel>(
        predicate: #Predicate<NoteModel> { note in
          note.frontMatter.userDefinedId == _id
        }
      )
      : FetchDescriptor<NoteModel>(
        predicate: #Predicate<NoteModel> { note in
          note.id == _id
        }
      )
    do {
      let res = try await context.fetch(fetchDescriptor)
      if res.isEmpty {
        print("No note found for id \(self.value)")
        return nil
      } else {
        return res.first
      }
    } catch {
      print("Error getting editing note: \(error)")
    }
    return nil
  }

  public static func fromString(_ val: String) -> EditingNoteId {
    return EditingNoteId(value: val)
  }

  public static func isEqual(_ val1: String, _ val2: String) -> Bool {
    return EditingNoteId.fromString(val1).id == EditingNoteId.fromString(val2).id
  }

  public static func isEqualToNote(_ val1: String, _ val2: NoteModel) -> Bool {
    let id = EditingNoteId.fromString(val1)
    if id.isUserDefined {
      return id.id == val2.frontMatter.userDefinedId
    } else {
      return id.id == val2.id
    }
  }
}
