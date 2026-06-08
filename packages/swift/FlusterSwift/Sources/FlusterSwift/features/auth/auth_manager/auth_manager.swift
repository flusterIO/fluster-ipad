//
//  File.swift
//  FlusterSwift
//
//  Created by Andrew on 6/8/26.
//

import StoreKit
import SwiftUI


@MainActor
public class AuthManager: ObservableObject {
    public var products: [Product] = []
    var purchasedProductIDs: Set<String> = []
    var isLoading = false
    var err: String? = nil
    private let productIds = [
        YEARLY_PRO,
        MONTHLY_PRO
    ]
    private var transactionListener: Task<Void, Error>? = nil
    public var hasActiveSubscriptions: Bool {
        !purchasedProductIDs.isEmpty
    }
    public init() {
        transactionListener = listenForTransactions()
        Task {
            await fetchProductData()
            await updatePurchased()
        }
    }
    
    deinit {
        transactionListener?.cancel()
    }
    
    private func checkVerified<T>(_ result: VerificationResult<T>) throws -> T {
        switch result {
        case .unverified:
            throw StoreError.failedVerification
        case .verified(let value):
            return value
        }
    }
    
    private func updatePurchased() async {
        for await result in Transaction.currentEntitlements {
            guard let transaction = try? checkVerified(result) else { continue}
            print(transaction)
            if transaction.revocationDate == nil {
                purchasedProductIDs.insert(transaction.productID)
            } else {
                purchasedProductIDs.remove(transaction.productID)
            }
        }
    }
    
    private func fetchProductData() async {
        isLoading = true
        do {
            products = try! await Product.products(for: productIds)
            products.sort { $0.price < $1.price }
        } catch {
            self.err = error.localizedDescription
        }
        isLoading = false
    }
    
    public func purchase(_ product: Product) async throws -> StoreKit.Transaction? {
        let result = try await product.purchase()
        
        switch result {
        case .pending:
            return nil
        case .success(let r ):
            let transaction = try checkVerified(r)
            await updatePurchased()
            await transaction.finish()
            return transaction
        case .userCancelled:
            return nil
        @unknown default:
            return nil
        }
    }
    
    private func restorePurchases() async {
        do {
            try await AppStore.sync()
            await updatePurchased()
        } catch {
            err = error.localizedDescription
        }
    }
    
    private func listenForTransactions() -> Task<Void, Error> {
        Task {
            for await update in Transaction.updates {
                if case .verified(let transaction ) = update {
                    await self.updatePurchased()
                    await transaction.finish()
                }
            }
        }
    }
}


