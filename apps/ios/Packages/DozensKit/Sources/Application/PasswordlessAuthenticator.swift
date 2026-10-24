import Domain

/// メールアドレスと OTP だけでサインイン/サインアップを 1 つのフローにまとめる。
public struct PasswordlessAuthenticator: Sendable {
    private let repository: any AuthRepository

    public init(repository: any AuthRepository) {
        self.repository = repository
    }

    public func start(email: Email) async throws(AuthError) -> PendingVerification {
        do {
            return .signIn(try await repository.signIn(email: email))
        } catch {
            switch error {
            case .userNotFound, .userNotConfirmed:
                break
            default:
                throw error
            }
        }
        try await repository.signUp(email: email)
        return .signUp(email)
    }

    public func verify(
        _ pending: PendingVerification,
        code: OneTimeCode
    ) async throws(AuthError) -> AuthTokens {
        switch pending {
        case .signIn(let challenge):
            try await repository.confirmSignIn(challenge: challenge, code: code)
        case .signUp(let email):
            try await repository.confirmSignUp(email: email, code: code)
        }
    }
}
