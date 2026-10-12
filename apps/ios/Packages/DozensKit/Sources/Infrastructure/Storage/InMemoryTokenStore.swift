import Domain
import Synchronization

public final class InMemoryTokenStore: TokenStore {
    private let tokens: Mutex<AuthTokens?>

    public init(tokens: AuthTokens? = nil) {
        self.tokens = Mutex(tokens)
    }

    public func load() throws -> AuthTokens? {
        tokens.withLock { $0 }
    }

    public func save(_ tokens: AuthTokens) throws {
        self.tokens.withLock { $0 = tokens }
    }

    public func clear() throws {
        tokens.withLock { $0 = nil }
    }
}
