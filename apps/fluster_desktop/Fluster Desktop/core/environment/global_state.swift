//
//  global_state.swift
//  Fluster
//
//  Created by Andrew on 1/18/26.
//

import Foundation
import Observation
import FlusterData
import Combine

public class AppState: ObservableObject {
    static let shared = AppState()
    @Published var mainView: MainViewKey = .dashboard
    @Published var editingNote: NoteModel?
}
