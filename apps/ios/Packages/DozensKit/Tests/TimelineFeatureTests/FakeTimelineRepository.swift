import Domain
import Foundation
import Synchronization

final class FakeTimelineRepository: TimelineRepository {
    /// フィードごとのページ。カーソルはページ番号の文字列。
    let pages: [TimelineFeed: [[TimelineEntry]]]
    let cypher: Cypher
    private let failure: Mutex<TimelineError?>

    init(pages: [TimelineFeed: [[TimelineEntry]]] = [:], cypher: Cypher = .sample(), failure: TimelineError? = nil) {
        self.pages = pages
        self.cypher = cypher
        self.failure = Mutex(failure)
    }

    func fail(with error: TimelineError?) {
        failure.withLock { $0 = error }
    }

    func timeline(_ feed: TimelineFeed, cursor: String?, accessToken: String) async throws(TimelineError) -> TimelinePage {
        if let error = failure.withLock({ $0 }) { throw error }
        let feedPages = pages[feed] ?? [[]]
        let index = cursor.flatMap(Int.init) ?? 0
        return TimelinePage(entries: feedPages[index], nextCursor: index + 1 < feedPages.count ? String(index + 1) : nil)
    }

    func cypher(id: Cypher.ID, accessToken: String) async throws(TimelineError) -> Cypher {
        if let error = failure.withLock({ $0 }) { throw error }
        guard id == cypher.id else { throw .notFound }
        return cypher
    }
}

struct FakeTokenStore: TokenStore {
    func load() throws -> AuthTokens? { AuthTokens(accessToken: "access", refreshToken: "refresh", idToken: "id") }
    func save(_ tokens: AuthTokens) throws {}
    func clear() throws {}
}

extension Rap {
    static func sample(_ id: String, cypherID: Cypher.ID = "cypher") -> Rap {
        Rap(
            id: id,
            cypherID: cypherID,
            rapper: Rapper(id: "rapper", name: "rapper"),
            videoURL: URL(filePath: "/dev/null"),
            createdAt: Date(timeIntervalSince1970: 0)
        )
    }
}

extension TimelineEntry {
    static func sample(_ id: String) -> TimelineEntry {
        TimelineEntry(kind: .rap, message: "", rap: .sample(id), beat: Beat(id: "beat", title: "beat"))
    }
}

extension Cypher {
    static func sample(raps: [String] = ["a", "b", "c"]) -> Cypher {
        Cypher(id: "cypher", beat: Beat(id: "beat", title: "beat"), raps: raps.map { Rap.sample($0) })
    }
}
