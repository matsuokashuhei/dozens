import Domain

public struct APIAuthRepository: AuthRepository {
    private let client: APIClient

    public init(client: APIClient) {
        self.client = client
    }

    public func signUp(email: Email) async throws(AuthError) {
        _ = try await send(AuthEndpoints.signUp(email: email))
    }

    public func confirmSignUp(email: Email, code: OneTimeCode) async throws(AuthError) -> AuthTokens {
        AuthTokens(try await send(AuthEndpoints.confirmSignUp(email: email, code: code)))
    }

    public func signIn(email: Email) async throws(AuthError) -> SignInChallenge {
        let response = try await send(AuthEndpoints.signIn(email: email))
        return SignInChallenge(email: email, session: response.session)
    }

    public func confirmSignIn(
        challenge: SignInChallenge,
        code: OneTimeCode
    ) async throws(AuthError) -> AuthTokens {
        AuthTokens(try await send(AuthEndpoints.confirmSignIn(challenge: challenge, code: code)))
    }

    public func signOut(accessToken: String) async throws(AuthError) {
        _ = try await send(AuthEndpoints.signOut(accessToken: accessToken))
    }

    public func currentUser(accessToken: String) async throws(AuthError) -> User {
        let response = try await send(AuthEndpoints.user(accessToken: accessToken))
        return User(id: response.user.id, name: response.user.name)
    }

    private func send<Response>(_ endpoint: Endpoint<Response>) async throws(AuthError) -> Response {
        do {
            return try await client.send(endpoint)
        } catch {
            throw AuthError(error)
        }
    }
}

extension AuthError {
    init(_ error: APIError) {
        switch error {
        case .transport:
            self = .network
        case .decoding(let message):
            self = .unexpected(message)
        case .status(code: 401, _):
            self = .unauthorized
        case .status(_, let message):
            self = AuthError(serverMessage: message)
        }
    }

    /// `apps/api` の `UsecaseError` の Display 文字列に対応する。
    init(serverMessage: String) {
        switch serverMessage.trimmingCharacters(in: .whitespacesAndNewlines) {
        case "invalid parameter": self = .invalidParameter
        case "user not found": self = .userNotFound
        case "user not confirmed": self = .userNotConfirmed
        case "user already exists": self = .userAlreadyExists
        case "code delivery failure": self = .codeDeliveryFailure
        case "code mismatch": self = .codeMismatch
        case "expired code": self = .expiredCode
        case "not authorized", "unauthorized": self = .unauthorized
        case let message where message.contains("invalid email"): self = .invalidParameter
        case let message: self = .unexpected(message)
        }
    }
}
