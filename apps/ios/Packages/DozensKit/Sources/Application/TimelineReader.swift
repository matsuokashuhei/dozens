import Domain

/// 保存済みのアクセストークンを付けてタイムラインとサイファーを取得する。
public struct TimelineReader: Sendable {
    private let repository: any TimelineRepository
    private let tokenStore: any TokenStore

    public init(repository: any TimelineRepository, tokenStore: any TokenStore) {
        self.repository = repository
        self.tokenStore = tokenStore
    }

    public func page(of feed: TimelineFeed, after cursor: String?) async throws(TimelineError) -> TimelinePage {
        try await repository.timeline(feed, cursor: cursor, accessToken: accessToken())
    }

    public func cypher(id: Cypher.ID) async throws(TimelineError) -> Cypher {
        try await repository.cypher(id: id, accessToken: accessToken())
    }

    private func accessToken() throws(TimelineError) -> String {
        guard let tokens = try? tokenStore.load() else { throw .unauthorized }
        return tokens.accessToken
    }
}
