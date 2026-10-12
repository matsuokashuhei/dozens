import Application
import Domain
import Testing

struct PasswordlessAuthenticatorTests {
    let email = try! Email("rapper@example.com")

    @Test func signsInExistingUser() async throws {
        let repository = FakeAuthRepository()
        let pending = try await PasswordlessAuthenticator(repository: repository).start(email: email)
        #expect(pending == .signIn(SignInChallenge(email: email, session: "session")))
        #expect(repository.calls.signUp.isEmpty)
    }

    @Test(arguments: [AuthError.userNotFound, .userNotConfirmed])
    func fallsBackToSignUp(error: AuthError) async throws {
        let repository = FakeAuthRepository(signInError: error)
        let pending = try await PasswordlessAuthenticator(repository: repository).start(email: email)
        #expect(pending == .signUp(email))
        #expect(repository.calls.signUp == [email])
    }

    @Test func propagatesOtherErrors() async {
        let repository = FakeAuthRepository(signInError: .codeDeliveryFailure)
        await #expect(throws: AuthError.codeDeliveryFailure) {
            try await PasswordlessAuthenticator(repository: repository).start(email: email)
        }
        #expect(repository.calls.signUp.isEmpty)
    }

    @Test(arguments: [
        PendingVerification.signIn(SignInChallenge(email: try! Email("rapper@example.com"), session: "s")),
        .signUp(try! Email("rapper@example.com")),
    ])
    func verifiesCode(pending: PendingVerification) async throws {
        let repository = FakeAuthRepository()
        let authenticator = PasswordlessAuthenticator(repository: repository)
        let tokens = try await authenticator.verify(pending, code: OneTimeCode("123456"))
        #expect(tokens == repository.tokens)
        await #expect(throws: AuthError.codeMismatch) {
            try await authenticator.verify(pending, code: OneTimeCode("000000"))
        }
    }
}
