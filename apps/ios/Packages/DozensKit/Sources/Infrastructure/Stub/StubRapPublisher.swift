import Domain
import Foundation

public struct StubRapPublisher: RapPublisher {
    private let latency: Duration

    public init(latency: Duration = .seconds(1)) {
        self.latency = latency
    }

    public func publish(_ draft: RapDraft, accessToken: String) async throws(RecordingError) -> Rap.ID {
        try? await Task.sleep(for: latency)
        guard FileManager.default.fileExists(atPath: draft.videoURL.path(percentEncoded: false)) else {
            throw .unexpected("video not found: \(draft.videoURL.lastPathComponent)")
        }
        return "rap-\(UUID().uuidString.lowercased())"
    }
}
