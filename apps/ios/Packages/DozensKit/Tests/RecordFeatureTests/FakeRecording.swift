import Domain
import Foundation
import Synchronization

final class FakeBeatRepository: BeatRepository {
    let catalog: [Beat]
    private let failure: Mutex<RecordingError?>

    init(catalog: [Beat] = [.sample("beat-a"), .sample("beat-b")], failure: RecordingError? = nil) {
        self.catalog = catalog
        self.failure = Mutex(failure)
    }

    func beats(accessToken: String) async throws(RecordingError) -> [Beat] {
        if let error = failure.withLock({ $0 }) { throw error }
        return catalog
    }

    func beat(id: Beat.ID, accessToken: String) async throws(RecordingError) -> Beat {
        if let error = failure.withLock({ $0 }) { throw error }
        guard let beat = catalog.first(where: { $0.id == id }) else { throw .notFound }
        return beat
    }
}

final class FakeRapPublisher: RapPublisher {
    let published = Mutex<[RapDraft]>([])
    private let failure: RecordingError?

    init(failure: RecordingError? = nil) {
        self.failure = failure
    }

    func publish(_ draft: RapDraft, accessToken: String) async throws(RecordingError) -> Rap.ID {
        if let failure { throw failure }
        published.withLock { $0.append(draft) }
        return "new-rap"
    }
}

struct FakeTokenStore: TokenStore {
    func load() throws -> AuthTokens? { AuthTokens(accessToken: "access", refreshToken: "refresh", idToken: "id") }
    func save(_ tokens: AuthTokens) throws {}
    func clear() throws {}
}

extension Beat {
    nonisolated static func sample(_ id: String) -> Beat {
        Beat(id: id, title: id, parts: [BeatPart(id: "\(id)-verse", name: "Verse", audioURL: URL(filePath: "/dev/null"), duration: .milliseconds(300))])
    }
}
