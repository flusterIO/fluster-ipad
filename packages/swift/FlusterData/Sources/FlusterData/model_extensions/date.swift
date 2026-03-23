//
//  File.swift
//  FlusterData
//
//  Created by Andrew on 3/23/26.
//

import Foundation

public extension Date {
    public func toRelativeTimeString(unitsStyle: RelativeDateTimeFormatter.UnitsStyle = .full, dateTimeStyle: RelativeDateTimeFormatter.DateTimeStyle = .named, relativeTo: Date = .now) -> String {
    let dateFormatter = RelativeDateTimeFormatter()
    dateFormatter.unitsStyle = unitsStyle
    dateFormatter.dateTimeStyle = dateTimeStyle
    return dateFormatter.localizedString(
      for: self,
      relativeTo: relativeTo
    )
  }
}
