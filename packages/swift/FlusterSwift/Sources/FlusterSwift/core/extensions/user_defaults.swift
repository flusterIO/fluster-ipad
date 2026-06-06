import SwiftUI

public extension UserDefaults {
    static func resetUserDefaultsDEVELOPMENTONLY() {
        let domain = Bundle.main.bundleIdentifier!
        UserDefaults.standard.removePersistentDomain(forName: domain)
        UserDefaults.standard.synchronize()
    }
}
