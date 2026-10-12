public protocol AuthRepository: Sendable {
    func signUp(email: Email) async throws(AuthError)
    func confirmSignUp(email: Email, code: OneTimeCode) async throws(AuthError) -> AuthTokens
    func signIn(email: Email) async throws(AuthError) -> SignInChallenge
    func confirmSignIn(challenge: SignInChallenge, code: OneTimeCode) async throws(AuthError) -> AuthTokens
    func signOut(accessToken: String) async throws(AuthError)
    func currentUser(accessToken: String) async throws(AuthError) -> User
}
