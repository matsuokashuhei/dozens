import Domain
import Foundation

/// API なしで UI を動かすための実装。OTP は `StubAuthRepository.validCode` だけを受け付ける。
public struct StubAuthRepository: AuthRepository {
    public static let validCode = "123456"
    public static let user = User(
        id: UUID(uuidString: "01960000-0000-7000-8000-000000000000")!,
        name: "dozens"
    )

    private let latency: Duration

    public init(latency: Duration = .milliseconds(400)) {
        self.latency = latency
    }

    public func signUp(email: Email) async throws(AuthError) {
        await wait()
    }

    public func confirmSignUp(email: Email, code: OneTimeCode) async throws(AuthError) -> AuthTokens {
        try await verify(code)
    }

    public func signIn(email: Email) async throws(AuthError) -> SignInChallenge {
        await wait()
        return SignInChallenge(email: email, session: "stub-session")
    }

    public func confirmSignIn(
        challenge: SignInChallenge,
        code: OneTimeCode
    ) async throws(AuthError) -> AuthTokens {
        try await verify(code)
    }

    public func signOut(accessToken: String) async throws(AuthError) {
        await wait()
    }

    public func currentUser(accessToken: String) async throws(AuthError) -> User {
        await wait()
        guard accessToken == Self.tokens.accessToken else { throw .unauthorized }
        return Self.user
    }

    private static let tokens = AuthTokens(
        accessToken: "stub-access-token",
        refreshToken: "stub-refresh-token",
        idToken: "stub-id-token"
    )

    private func verify(_ code: OneTimeCode) async throws(AuthError) -> AuthTokens {
        await wait()
        guard code.rawValue == Self.validCode else { throw .codeMismatch }
        return Self.tokens
    }

    private func wait() async {
        try? await Task.sleep(for: latency)
    }
}
