import Application
import Domain
import Testing

@MainActor
struct SessionStoreTests {
    @Test func restoresSignedOutWithoutTokens() async {
        let store = SessionStore(repository: FakeAuthRepository(), tokenStore: FakeTokenStore())
        await store.restore()
        #expect(store.state == .signedOut)
    }

    @Test func restoresSignedInWithValidTokens() async {
        let repository = FakeAuthRepository()
        let store = SessionStore(repository: repository, tokenStore: FakeTokenStore(repository.tokens))
        await store.restore()
        #expect(store.state == .signedIn(repository.user))
    }

    @Test func clearsTokensWhenUnauthorized() async throws {
        let repository = FakeAuthRepository(currentUserError: .unauthorized)
        let tokenStore = FakeTokenStore(repository.tokens)
        let store = SessionStore(repository: repository, tokenStore: tokenStore)
        await store.restore()
        #expect(store.state == .signedOut)
        #expect(try tokenStore.load() == nil)
    }

    @Test func keepsTokensOnNetworkError() async throws {
        let repository = FakeAuthRepository(currentUserError: .network)
        let tokenStore = FakeTokenStore(repository.tokens)
        let store = SessionStore(repository: repository, tokenStore: tokenStore)
        await store.restore()
        #expect(store.state == .signedOut)
        #expect(try tokenStore.load() == repository.tokens)
    }

    @Test func signInSavesTokensAndSignOutRevokesThem() async throws {
        let repository = FakeAuthRepository()
        let tokenStore = FakeTokenStore()
        let store = SessionStore(repository: repository, tokenStore: tokenStore)

        try await store.signIn(with: repository.tokens)
        #expect(store.state == .signedIn(repository.user))
        #expect(try tokenStore.load() == repository.tokens)

        await store.signOut()
        #expect(store.state == .signedOut)
        #expect(try tokenStore.load() == nil)
        #expect(repository.calls.signOut == ["access"])
    }
}
