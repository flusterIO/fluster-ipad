//
//  paywall_view.swift
//  Fluster
//
//  Created by Andrew on 6/8/26.
//

import Foundation
import FlusterSwift
import StoreKit
import SwiftUI

struct PaywallView: View {
    @EnvironmentObject private var store: AuthManager
    @Environment(\.dismiss) private var dismiss
    @State private var seleectedProduct: Product?
    @State private var isPurchasing = false
    @State private var showError = false
    public init() {}
    var body: some View {
        Text("Paywall here...")
    }
}

#Preview {
    PaywallView()
        .environmentObject(AuthManager())
}
