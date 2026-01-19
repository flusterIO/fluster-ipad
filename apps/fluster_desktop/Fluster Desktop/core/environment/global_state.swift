//
//  global_state.swift
//  Fluster
//
//  Created by Andrew on 1/18/26.
//

import Foundation
import Observation

@Observable
class AppState {
    var mainView: MainViewKey = .dashboard
}
