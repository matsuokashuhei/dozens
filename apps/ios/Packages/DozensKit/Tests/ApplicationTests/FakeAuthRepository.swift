import Domain
import Foundation
import Synchronization

final class FakeAuthRepository: AuthRepository {
    struct Calls: Equatable {
        var signUp: [Email] = []
        var signIn: [Email] = []
        var signOut: [String] = []
    }

    let user = User(id: UUID(), name: "tester")
    let tokens = AuthTokens(accessToken: "access", refreshToken: "refresh", idToken: "id")

    private let state: Mutex<(calls: Calls, signInError: AuthError?, currentUserError: AuthError?)>

    init(signInError: AuthError? = nil, currentUserError: AuthError? = nil) {
        state = Mutex((Calls(), signInError, currentUserError))
    }

    var calls: Calls { state.withLock { $0.calls } }

    func signUp(email: Email) async throws(AuthError) {
        state.withLock { $0.calls.signUp.append(email) }
    }

    func confirmSignUp(email: Email, code: OneTimeCode) async throws(AuthError) -> AuthTokens {
        try check(code)
    }

    func signIn(email: Email) async throws(AuthError) -> SignInChallenge {
        let error = state.withLock { state in
            state.calls.signIn.append(email)
            return state.signInError
        }
        if let error { throw error }
        return SignInChallenge(email: email, session: "session")
    }

    func confirmSignIn(challenge: SignInChallenge, code: OneTimeCode) async throws(AuthError) -> AuthTokens {
        try check(code)
    }

    func signOut(accessToken: String) async throws(AuthError) {
        state.withLock { $0.calls.signOut.append(accessToken) }
    }

    func currentUser(accessToken: String) async throws(AuthError) -> User {
        if let error = state.withLock({ $0.currentUserError }) { throw error }
        return user
    }

    private func check(_ code: OneTimeCode) throws(AuthError) -> AuthTokens {
        guard code.rawValue == "123456" else { throw .codeMismatch }
        return tokens
    }
}

final class FakeTokenStore: TokenStore {
    private let tokens: Mutex<AuthTokens?>

    init(_ tokens: AuthTokens? = nil) {
        self.tokens = Mutex(tokens)
    }

    func load() throws -> AuthTokens? { tokens.withLock { $0 } }
    func save(_ tokens: AuthTokens) throws { self.tokens.withLock { $0 = tokens } }
    func clear() throws { tokens.withLock { $0 = nil } }
}
