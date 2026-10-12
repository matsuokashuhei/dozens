import Domain
import Foundation
import Infrastructure
import Testing

struct APIAuthRepositoryTests {
    private func repository(
        _ host: String = UUID().uuidString.lowercased(),
        handler: @escaping StubURLProtocol.Handler
    ) -> APIAuthRepository {
        let session = StubURLProtocol.session(host: host, handler: handler)
        return APIAuthRepository(client: APIClient(baseURL: URL(string: "http://\(host)")!, session: session))
    }

    private func json(_ body: Data?) -> [String: String] {
        (try? JSONSerialization.jsonObject(with: body ?? Data()) as? [String: String]) ?? [:]
    }

    @Test func signInPostsEmailAndReturnsChallenge() async throws {
        let repository = repository { request in
            #expect(request.httpMethod == "POST")
            #expect(request.url?.path() == "/sign_in")
            #expect(request.value(forHTTPHeaderField: "Content-Type") == "application/json")
            #expect(self.json(request.httpBody) == ["email": "rapper@example.com"])
            return (200, Data(#"{"email":"rapper@example.com","session":"abc"}"#.utf8))
        }
        let email = try Email("rapper@example.com")
        let challenge = try await repository.signIn(email: email)
        #expect(challenge == SignInChallenge(email: email, session: "abc"))
    }

    @Test func confirmSignInSendsSessionAndDecodesTokens() async throws {
        let repository = repository { request in
            #expect(request.url?.path() == "/confirm_sign_in")
            #expect(self.json(request.httpBody) == ["session": "abc", "email": "rapper@example.com", "code": "123456"])
            return (200, Data(#"{"access_token":"a","refresh_token":"r","id_token":"i"}"#.utf8))
        }
        let challenge = SignInChallenge(email: try Email("rapper@example.com"), session: "abc")
        let tokens = try await repository.confirmSignIn(challenge: challenge, code: OneTimeCode("123456"))
        #expect(tokens == AuthTokens(accessToken: "a", refreshToken: "r", idToken: "i"))
    }

    @Test func currentUserSendsBearerToken() async throws {
        let id = UUID()
        let repository = repository { request in
            #expect(request.httpMethod == "GET")
            #expect(request.url?.path() == "/user")
            #expect(request.value(forHTTPHeaderField: "Authorization") == "Bearer token")
            return (200, Data(#"{"user":{"id":"\#(id.uuidString)","name":"mc"}}"#.utf8))
        }
        #expect(try await repository.currentUser(accessToken: "token") == User(id: id, name: "mc"))
    }

    @Test func signOutAcceptsNoContent() async throws {
        let repository = repository { request in
            #expect(request.httpMethod == "DELETE")
            #expect(request.url?.path() == "/sign_out")
            return (204, Data())
        }
        try await repository.signOut(accessToken: "token")
    }

    @Test(arguments: [
        ("user not found", 400, AuthError.userNotFound),
        ("user not confirmed", 400, .userNotConfirmed),
        ("code mismatch", 400, .codeMismatch),
        ("expired code", 400, .expiredCode),
        ("Failed to deserialize the JSON body into the target type: email: invalid email", 422, .invalidParameter),
        ("unauthorized", 401, .unauthorized),
        ("boom", 500, .unexpected("boom")),
    ])
    func mapsServerErrors(message: String, status: Int, expected: AuthError) async {
        let repository = repository { _ in (status, Data(message.utf8)) }
        await #expect(throws: expected) {
            try await repository.signIn(email: Email("rapper@example.com"))
        }
    }

    @Test func mapsTransportFailureToNetwork() async {
        let session = StubURLProtocol.session(host: "registered.invalid") { _ in (200, Data()) }
        let client = APIClient(baseURL: URL(string: "http://unregistered.invalid")!, session: session)
        await #expect(throws: AuthError.network) {
            try await APIAuthRepository(client: client).signIn(email: Email("rapper@example.com"))
        }
    }
}
