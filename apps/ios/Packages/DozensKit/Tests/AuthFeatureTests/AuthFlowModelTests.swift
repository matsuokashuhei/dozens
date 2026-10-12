import Application
import AuthFeature
import Domain
import Foundation
import Synchronization
import Testing

@MainActor
struct AuthFlowModelTests {
    private func makeModel(signInError: AuthError? = nil) -> (AuthFlowModel, SessionStore) {
        let repository = Repository(signInError: signInError)
        let session = SessionStore(repository: repository, tokenStore: Tokens())
        let model = AuthFlowModel(authenticator: PasswordlessAuthenticator(repository: repository), session: session)
        return (model, session)
    }

    @Test func emailSubmissionMovesToCodeStep() async {
        let (model, _) = makeModel()
        model.begin()
        model.emailText = "invalid"
        #expect(!model.canSubmitEmail)

        model.emailText = "rapper@example.com"
        #expect(model.canSubmitEmail)
        await model.submitEmail()

        #expect(model.path == [.email, .code])
        #expect(model.pending?.email.rawValue == "rapper@example.com")
    }

    @Test func codeIsSanitizedToSixDigits() {
        let (model, _) = makeModel()
        model.codeText = "12a3 4567"
        #expect(model.codeText == "123456")
    }

    @Test func correctCodeSignsIn() async {
        let (model, session) = makeModel()
        model.emailText = "rapper@example.com"
        await model.submitEmail()
        model.codeText = "123456"
        await model.submitCode()
        #expect(session.currentUser?.name == "tester")
        #expect(model.error == nil)
    }

    @Test func wrongCodeShowsErrorAndClearsInput() async {
        let (model, session) = makeModel()
        model.emailText = "rapper@example.com"
        await model.submitEmail()
        model.codeText = "000000"
        await model.submitCode()
        #expect(model.error == .codeMismatch)
        #expect(model.codeText.isEmpty)
        #expect(session.currentUser == nil)
    }

    @Test func emailErrorStaysOnEmailStep() async {
        let (model, _) = makeModel(signInError: .codeDeliveryFailure)
        model.begin()
        model.emailText = "rapper@example.com"
        await model.submitEmail()
        #expect(model.path == [.email])
        #expect(model.error == .codeDeliveryFailure)
    }
}

private struct Repository: AuthRepository {
    let signInError: AuthError?

    func signUp(email: Email) async throws(AuthError) {}

    func confirmSignUp(email: Email, code: OneTimeCode) async throws(AuthError) -> AuthTokens {
        try check(code)
    }

    func signIn(email: Email) async throws(AuthError) -> SignInChallenge {
        if let signInError { throw signInError }
        return SignInChallenge(email: email, session: "session")
    }

    func confirmSignIn(challenge: SignInChallenge, code: OneTimeCode) async throws(AuthError) -> AuthTokens {
        try check(code)
    }

    func signOut(accessToken: String) async throws(AuthError) {}

    func currentUser(accessToken: String) async throws(AuthError) -> User {
        User(id: UUID(), name: "tester")
    }

    private func check(_ code: OneTimeCode) throws(AuthError) -> AuthTokens {
        guard code.rawValue == "123456" else { throw .codeMismatch }
        return AuthTokens(accessToken: "a", refreshToken: "r", idToken: "i")
    }
}

private final class Tokens: TokenStore {
    private let tokens = Mutex<AuthTokens?>(nil)
    func load() throws -> AuthTokens? { tokens.withLock { $0 } }
    func save(_ tokens: AuthTokens) throws { self.tokens.withLock { $0 = tokens } }
    func clear() throws { tokens.withLock { $0 = nil } }
}
