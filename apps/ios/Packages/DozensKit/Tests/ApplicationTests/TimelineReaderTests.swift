import Application
import Domain
import Testing

struct TimelineReaderTests {
    @Test func requiresStoredToken() async {
        let reader = TimelineReader(repository: UnreachableTimelineRepository(), tokenStore: FakeTokenStore())
        await #expect(throws: TimelineError.unauthorized) {
            try await reader.page(of: .following, after: nil)
        }
    }
}

private struct UnreachableTimelineRepository: TimelineRepository {
    func timeline(_ feed: TimelineFeed, cursor: String?, accessToken: String) async throws(TimelineError) -> TimelinePage {
        Issue.record("repository must not be called without a token")
        return TimelinePage(entries: [], nextCursor: nil)
    }

    func cypher(id: Cypher.ID, accessToken: String) async throws(TimelineError) -> Cypher {
        Issue.record("repository must not be called without a token")
        throw .notFound
    }
}
