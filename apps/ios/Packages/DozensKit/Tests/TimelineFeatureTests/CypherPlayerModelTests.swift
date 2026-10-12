import Application
import Domain
import Testing
import TimelineFeature

struct CypherPlayerModelTests {
    private func makeModel(startRapID: Rap.ID, repository: FakeTimelineRepository = FakeTimelineRepository()) -> CypherPlayerModel {
        CypherPlayerModel(
            cypherID: "cypher",
            startRapID: startRapID,
            reader: TimelineReader(repository: repository, tokenStore: FakeTokenStore())
        )
    }

    @Test func startsFromSelectedRap() async {
        let model = makeModel(startRapID: "b")
        await model.load()
        #expect(model.currentRapID == "b")
    }

    @Test func fallsBackToFirstRapWhenSelectedRapIsGone() async {
        let model = makeModel(startRapID: "deleted")
        await model.load()
        #expect(model.currentRapID == "a")
    }

    @Test func advanceWrapsAround() async {
        let model = makeModel(startRapID: "b")
        await model.load()
        model.advance()
        #expect(model.currentRapID == "c")
        model.advance()
        #expect(model.currentRapID == "a")
    }

    @Test func reportsLoadError() async {
        let model = makeModel(startRapID: "a", repository: FakeTimelineRepository(failure: .notFound))
        await model.load()
        #expect(model.cypher == nil)
        #expect(model.error == .notFound)
    }
}
