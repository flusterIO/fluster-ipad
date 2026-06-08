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
        VStack(alignment: .leading) {
            HStack {
                Image("flusterIcon")
                    .resizable()
                    .foregroundStyle(.tint)
                    .aspectRatio(contentMode: .fit)
                    .frame(width: 48, height: 48, alignment: .center)
            Text("Fluster Pro")
                .font(.largeTitle)
            }
            Text("If you see the potential in Fluster and Conundrum, please consider subscribing. You'll get Unlimited notes, immediate access to the mobile app, and you'll be supporting the future growth of the Conundrum ecosystem.")
                .frame(maxWidth: 350)
            ForEach(store.products, id: \.id) { product in
                Button( action: {
                    Task {
                        do {
                           let res = try await store.purchase(product)
                        } catch {
                            print("Purchase Error... and you haven't had a paycheck in 5 years. \(error.localizedDescription)")
                        }
                    }
                }, label: {
                    Text("\(getButtonText(product.id)) \(product.displayPrice)")
                })
            }
        }
    }
    
    func getButtonText(_ id: String) -> String {
        switch id {
        case MONTHLY_PRO:
            return "Fluster Pro Monthly"
        case YEARLY_PRO:
            return "Fluster Pro Yearly"
        default:
            return "Fluster Pro"
        }
    }
}

#Preview {
    PaywallView()
        .environmentObject(AuthManager())
}
