import Application
import Domain
import Foundation
import RecordFeature
import Testing

struct RecordModelTests {
    private func makeModel(beats: FakeBeatRepository = FakeBeatRepository(), publisher: FakeRapPublisher = FakeRapPublisher()) -> RecordModel {
        RecordModel(service: RecordingService(beatRepository: beats, publisher: publisher, tokenStore: FakeTokenStore()))
    }

    @Test func loadsAllBeats() async {
        let model = makeModel()
        await model.load()
        #expect(model.beats.map(\.id) == ["beat-a", "beat-b"])
        #expect(model.isLoaded)
    }

    @Test func hookUpLoadsOnlyTheCypherBeat() async {
        let model = makeModel()
        model.start(hookUp: .init(rapID: "rap-1", beatID: "beat-b"))
        await model.load()
        #expect(model.beats.map(\.id) == ["beat-b"])
    }

    @Test func reportsLoadError() async {
        let model = makeModel(beats: FakeBeatRepository(failure: .network))
        await model.load()
        #expect(model.error == .network)
        #expect(model.isLoaded)
        #expect(model.beats.isEmpty)
    }

    @Test func recordingProducesDraftForHookUp() {
        let model = makeModel()
        let part = Beat.sample("beat-a").parts[0]
        model.start(hookUp: .init(rapID: "rap-1", beatID: "beat-a"))
        model.select(part)
        #expect(model.step == .recording(part))
        let video = URL(filePath: "/tmp/take.mov")
        model.finishRecording(videoURL: video, part: part)
        #expect(model.step == .review(RapDraft(videoURL: video, beatPartID: part.id, hookUpRapID: "rap-1")))
    }

    @Test func discardReturnsToBeats() {
        let model = makeModel()
        model.select(Beat.sample("beat-a").parts[0])
        model.discardRecording()
        #expect(model.step == .beats)
    }

    @Test func publishSendsDraftAndClearsHookUp() async {
        let publisher = FakeRapPublisher()
        let model = makeModel(publisher: publisher)
        model.start(hookUp: .init(rapID: "rap-1", beatID: "beat-a"))
        let draft = RapDraft(videoURL: URL(filePath: "/tmp/take.mov"), beatPartID: "beat-a-verse", hookUpRapID: "rap-1", lyrics: "yo", tags: ["tokyo"])
        await model.publish(draft)
        #expect(model.step == .published("new-rap"))
        #expect(model.hookUp == nil)
        #expect(publisher.published.withLock { $0 } == [draft])
    }

    @Test func publishFailureKeepsReview() async {
        let model = makeModel(publisher: FakeRapPublisher(failure: .network))
        let draft = RapDraft(videoURL: URL(filePath: "/tmp/take.mov"), beatPartID: "beat-a-verse")
        model.step = .review(draft)
        await model.publish(draft)
        #expect(model.error == .network)
        #expect(model.step == .review(draft))
        #expect(!model.isPublishing)
    }

    @Test(arguments: [
        ("#tokyo #freestyle", ["tokyo", "freestyle"]),
        ("tokyo, freestyle、tokyo", ["tokyo", "freestyle"]),
        ("  ＃日本語  ##x ", ["日本語", "x"]),
        ("", []),
    ])
    func parsesTags(text: String, expected: [String]) {
        #expect(RecordModel.tags(from: text) == expected)
    }
}
