/// API の `UsecaseError` に対応する認証エラー。
public enum AuthError: Error, Equatable, Sendable {
    case invalidParameter
    case userNotFound
    case userNotConfirmed
    case userAlreadyExists
    case codeDeliveryFailure
    case codeMismatch
    case expiredCode
    case unauthorized
    case network
    case unexpected(String)
}
