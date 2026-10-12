import Domain
import Foundation

extension AuthError {
    var message: String {
        switch self {
        case .invalidParameter:
            String(localized: "The email address or code is invalid.", bundle: .module)
        case .userNotFound:
            String(localized: "No account was found for this email address.", bundle: .module)
        case .userNotConfirmed:
            String(localized: "This account has not been confirmed yet.", bundle: .module)
        case .userAlreadyExists:
            String(localized: "An account with this email address already exists.", bundle: .module)
        case .codeDeliveryFailure:
            String(localized: "We couldn't send the code. Please try again later.", bundle: .module)
        case .codeMismatch:
            String(localized: "The code is incorrect.", bundle: .module)
        case .expiredCode:
            String(localized: "The code has expired. Please request a new one.", bundle: .module)
        case .unauthorized:
            String(localized: "Your session has expired. Please sign in again.", bundle: .module)
        case .network:
            String(localized: "Couldn't connect to the server.", bundle: .module)
        case .unexpected:
            String(localized: "Something went wrong. Please try again.", bundle: .module)
        }
    }
}
