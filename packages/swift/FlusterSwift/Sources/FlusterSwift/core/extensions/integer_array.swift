//
//  File.swift
//  FlusterSwift
//
//  Created by Andrew on 12/2/25.
//

import Foundation
import FlusterRust

extension [Int] {
    public func toData() -> Data {
        self.withUnsafeBufferPointer { buffer in
            Data(buffer: buffer)
        }
    }
}
