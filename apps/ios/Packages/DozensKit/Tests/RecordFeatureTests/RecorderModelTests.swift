import Domain
import Foundation
@testable import RecordFeature
import Testing

struct RecorderModelTests {
    private let part = BeatPart(id: "verse", name: "Verse", audioURL: URL(filePath: "/dev/null"), duration: .milliseconds(300))

    @Test func recordsForThePartDuration() async {
        let capture = StubCapture()
        let model = RecorderModel(part: part, capture: capture, countInBeat: .milliseconds(10))
        let url = await model.record()
        #expect(url == StubCapture.output)
        #expect(capture.started && capture.stopped)
        #expect(model.phase == .finished)
        #expect(model.elapsed >= part.duration)
    }

    @Test func stopDuringCountInCancels() async {
        let capture = StubCapture()
        let model = RecorderModel(part: part, capture: capture, countInBeat: .milliseconds(200))
        async let url = model.record()
        try? await Task.sleep(for: .milliseconds(50))
        model.stop()
        #expect(await url == nil)
        #expect(!capture.started)
        #expect(model.phase == .ready)
    }

    @Test func stopDuringRecordingFinishesEarly() async {
        let longPart = BeatPart(id: "long", name: "Long", audioURL: URL(filePath: "/dev/null"), duration: .seconds(30))
        let capture = StubCapture()
        let model = RecorderModel(part: longPart, capture: capture, countInBeat: .milliseconds(10))
        async let url = model.record()
        try? await Task.sleep(for: .milliseconds(300))
        model.stop()
        #expect(await url == StubCapture.output)
        #expect(model.elapsed < longPart.duration)
    }

    @Test func sampleCaptureCopiesBundledVideo() async throws {
        let url = try #require(await SampleVideoCapture().stop())
        #expect(FileManager.default.fileExists(atPath: url.path(percentEncoded: false)))
        try FileManager.default.removeItem(at: url)
    }
}

private final class StubCapture: VideoCapture {
    static let output = URL(filePath: "/tmp/take.mov")
    private(set) var started = false
    private(set) var stopped = false

    func start() { started = true }

    func stop() async -> URL? {
        stopped = true
        return Self.output
    }
}
