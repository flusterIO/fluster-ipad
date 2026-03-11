//
//  File.swift
//  FlusterSwift
//
//  Created by Andrew on 3/11/26.
//

import Foundation

let sharedEmbbededUrlHost = "fluster.internal"

public extension URL {
    public static func embeddedFlusterUrl(folder: String, fileName: String) -> URL {
        let url = URL(string: "app://\(sharedEmbbededUrlHost)/\(folder)/\(fileName)")!
        return url
    }
}
