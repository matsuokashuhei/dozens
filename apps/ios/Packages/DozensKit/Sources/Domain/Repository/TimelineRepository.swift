public protocol TimelineRepository: Sendable {
    func timeline(_ feed: TimelineFeed, cursor: String?, accessToken: String) async throws(TimelineError) -> TimelinePage
    func cypher(id: Cypher.ID, accessToken: String) async throws(TimelineError) -> Cypher
}
