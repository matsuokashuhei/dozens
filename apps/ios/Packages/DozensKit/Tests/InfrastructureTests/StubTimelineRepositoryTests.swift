import Domain
import Infrastructure
import Testing

struct StubTimelineRepositoryTests {
    private let repository = StubTimelineRepository(latency: .zero)

    @Test(arguments: TimelineFeed.allCases)
    func pagesThroughFeedWithoutDuplicates(feed: TimelineFeed) async throws {
        var ids: [Rap.ID] = []
        var cursor: String?
        repeat {
            let page = try await repository.timeline(feed, cursor: cursor, accessToken: "")
            #expect(page.entries.count <= StubTimelineRepository.pageSize)
            ids += page.entries.map(\.id)
            cursor = page.nextCursor
        } while cursor != nil
        #expect(!ids.isEmpty)
        #expect(Set(ids).count == ids.count)
    }

    @Test func entriesPointToPlayableCyphers() async throws {
        let entry = try #require(try await repository.timeline(.newArrivals, cursor: nil, accessToken: "").entries.first)
        let cypher = try await repository.cypher(id: entry.rap.cypherID, accessToken: "")
        #expect(cypher.raps.contains { $0.id == entry.rap.id })
        #expect(entry.rap.videoURL.isFileURL)
    }

    @Test func unknownCypherIsNotFound() async {
        await #expect(throws: TimelineError.notFound) {
            try await repository.cypher(id: "missing", accessToken: "")
        }
    }
}
