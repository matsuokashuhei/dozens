import Application
import Domain
import Testing
import TimelineFeature

struct TimelineModelTests {
    private func makeModel(_ repository: FakeTimelineRepository) -> TimelineModel {
        TimelineModel(reader: TimelineReader(repository: repository, tokenStore: FakeTokenStore()))
    }

    @Test func reloadLoadsFirstPage() async {
        let model = makeModel(FakeTimelineRepository(pages: [.following: [[.sample("1"), .sample("2")], [.sample("3")]]]))
        await model.reload()
        #expect(model.entries.map(\.id) == ["1", "2"])
        #expect(model.error == nil)
    }

    @Test func loadMoreAppendsNextPageOnlyAtTheEnd() async {
        let model = makeModel(FakeTimelineRepository(pages: [.following: [[.sample("1"), .sample("2")], [.sample("2"), .sample("3")]]]))
        await model.reload()

        await model.loadMore(ifReached: .sample("1"))
        #expect(model.entries.map(\.id) == ["1", "2"])

        await model.loadMore(ifReached: .sample("2"))
        #expect(model.entries.map(\.id) == ["1", "2", "3"])

        await model.loadMore(ifReached: .sample("3"))
        #expect(model.entries.map(\.id) == ["1", "2", "3"])
    }

    @Test func changingFeedClearsEntries() async {
        let model = makeModel(FakeTimelineRepository(pages: [.following: [[.sample("1")]], .featured: [[.sample("9")]]]))
        await model.reload()
        model.feed = .featured
        #expect(model.entries.isEmpty)
        await model.reload()
        #expect(model.entries.map(\.id) == ["9"])
    }

    @Test func failedReloadKeepsEntriesAndReportsError() async {
        let repository = FakeTimelineRepository(pages: [.following: [[.sample("1")]]])
        let model = makeModel(repository)
        await model.reload()
        repository.fail(with: .network)
        await model.reload()
        #expect(model.entries.map(\.id) == ["1"])
        #expect(model.error == .network)
    }
}
