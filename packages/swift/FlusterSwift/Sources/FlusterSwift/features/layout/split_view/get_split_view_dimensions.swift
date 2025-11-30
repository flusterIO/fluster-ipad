//
//  get_split_view_dimensions.swift
//  Fluster
//
//  Created by Andrew on 11/30/25.
//

import Foundation
import SwiftUI

func getSplitViewDimensions(drag: DragGesture.Value, rect: GeometryProxy)
    -> Double
{
    let newRatio = drag.location.x / rect.size.width
    let screenWidth = UIScreen.current?.bounds.width ?? 1

    return newRatio
}
