//
//  File.swift
//  FlusterSwift
//
//  Created by Andrew on 12/4/25.
//

import Foundation
import SwiftData

@ModelActor
public actor DataHandler {
  // -- Notes --
  @discardableResult
  public func saveNote(item: NoteModel) throws -> PersistentIdentifier {
    modelContext.insert(item)
    try modelContext.save()
    return item.persistentModelID
  }

  public func getNoteById(id: PersistentIdentifier) throws -> NoteModel? {
    guard let item = self[id, as: NoteModel.self] else { return nil }
    return item
  }

  public func deleteNote(id: PersistentIdentifier) throws {
    guard let item = self[id, as: NoteModel.self] else { return }
    modelContext.delete(item)
    try modelContext.save()
  }
  // -- Bibliography --
  //    public func getAllBibEntries() throws {
  //
  //    }
}
