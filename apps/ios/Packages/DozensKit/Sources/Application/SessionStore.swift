import Domain
import Observation

@MainActor
@Observable
public final class SessionStore {
    public enum State: Equatable, Sendable {
        case restoring
        case signedOut
        case signedIn(User)
    }

    public private(set) var state: State = .restoring

    private let repository: any AuthRepository
    private let tokenStore: any TokenStore

    public init(repository: any AuthRepository, tokenStore: any TokenStore) {
        self.repository = repository
        self.tokenStore = tokenStore
    }

    public var currentUser: User? {
        if case .signedIn(let user) = state { user } else { nil }
    }

    public func restore() async {
        guard let tokens = try? tokenStore.load() else {
            state = .signedOut
            return
        }
        do {
            state = .signedIn(try await repository.currentUser(accessToken: tokens.accessToken))
        } catch {
            if error == .unauthorized || error == .userNotFound {
                try? tokenStore.clear()
            }
            state = .signedOut
        }
    }

    public func signIn(with tokens: AuthTokens) async throws(AuthError) {
        let user = try await repository.currentUser(accessToken: tokens.accessToken)
        do {
            try tokenStore.save(tokens)
        } catch {
            throw .unexpected(String(describing: error))
        }
        state = .signedIn(user)
    }

    public func signOut() async {
        if let tokens = try? tokenStore.load() {
            try? await repository.signOut(accessToken: tokens.accessToken)
        }
        try? tokenStore.clear()
        state = .signedOut
    }
}
