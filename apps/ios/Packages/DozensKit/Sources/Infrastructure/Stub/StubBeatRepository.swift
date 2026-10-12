import Domain
import Foundation

public struct StubBeatRepository: BeatRepository {
    static let catalog: [Beat] = [
        makeBeat(id: "beat-0", title: "Midnight Tokyo"),
        makeBeat(id: "beat-1", title: "Boom Bap Classic"),
        makeBeat(id: "beat-2", title: "Trap Rain"),
    ]

    private let latency: Duration

    public init(latency: Duration = .milliseconds(400)) {
        self.latency = latency
    }

    public func beats(accessToken: String) async throws(RecordingError) -> [Beat] {
        try? await Task.sleep(for: latency)
        return Self.catalog
    }

    public func beat(id: Beat.ID, accessToken: String) async throws(RecordingError) -> Beat {
        try? await Task.sleep(for: latency)
        guard let beat = Self.catalog.first(where: { $0.id == id }) else { throw .notFound }
        return beat
    }

    private static let audioURL = Bundle.module.url(forResource: "stub-beat", withExtension: "m4a")!

    private static func makeBeat(id: String, title: String) -> Beat {
        let parts = ["Verse", "Hook"].map { (name: String) -> BeatPart in
            BeatPart(id: "\(id)-\(name.lowercased())", name: name, audioURL: audioURL, duration: .milliseconds(10_665))
        }
        return Beat(id: id, title: title, artistName: "Dozens Beats", parts: parts)
    }
}
