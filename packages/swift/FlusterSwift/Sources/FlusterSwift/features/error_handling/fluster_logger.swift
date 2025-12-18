//
//  File.swift
//  FlusterSwift
//
//  Created by Andrew on 12/17/25.
//

import OSLog
import SwiftUI

public enum FlusterLoggerSubdomain: String {
    case mainApp, appLaunch, serialization
}

public enum FlusterLoggerCategory: String {
    case devOnly, prodAndDev, prodOnly
}
public enum FlusterLoggerLevel: String {
    case debug, error, info, log, warning, trace, critical, fault
}

public class FlusterLogger {
    public let logger: Logger

    public init(_ subdomain: FlusterLoggerSubdomain, _ category: FlusterLoggerCategory) {
        self.logger = Logger(
            subsystem: subdomain.rawValue,
            category: category.rawValue
        )
    }

    public func log(_ value: String, _ level: FlusterLoggerLevel) {
        switch level {
        case .debug:
            self.logger.debug("\(value)")
        case .error:
            self.logger.error("\(value)")
        case .info:
            self.logger.info("\(value)")
        case .log:
            self.logger.log("\(value)")
        case .critical:
            self.logger.critical("\(value)")
        case .trace:
            self.logger.trace("\(value)")
        case .warning:
            self.logger.warning("\(value)")
        case .fault:
            self.logger.fault("\(value)")
        default:
            self.logger.log("\(value)")
        }
    }
}
