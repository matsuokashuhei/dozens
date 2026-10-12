import AVFoundation
import Domain
import Infrastructure
import Testing

struct StubBeatRepositoryTests {
    private let repository = StubBeatRepository(latency: .zero)

    @Test func everyBeatHasPlayableParts() async throws {
        let beats = try await repository.beats(accessToken: "token")
        #expect(!beats.isEmpty)
        for part in beats.flatMap(\.parts) {
            let duration = try await AVURLAsset(url: part.audioURL).load(.duration)
            #expect(duration.seconds > 10)
        }
    }

    @Test func unknownBeatIsNotFound() async {
        await #expect(throws: RecordingError.notFound) {
            try await repository.beat(id: "missing", accessToken: "token")
        }
    }

    @Test func publisherRejectsMissingVideo() async {
        let draft = RapDraft(videoURL: URL(filePath: "/nonexistent.mov"), beatPartID: "beat-0-verse")
        await #expect(throws: RecordingError.self) {
            try await StubRapPublisher(latency: .zero).publish(draft, accessToken: "token")
        }
    }
}
