import Domain
import Foundation

/// API にタイムラインが実装されるまでの仮実装。固定のサイファーから各フィードを組み立てる。
public struct StubTimelineRepository: TimelineRepository {
    public static let pageSize = 8

    private let cyphers: [Cypher]
    private let latency: Duration

    public init(now: Date = .now, latency: Duration = .milliseconds(400)) {
        cyphers = Self.makeCyphers(now: now)
        self.latency = latency
    }

    public func timeline(
        _ feed: TimelineFeed,
        cursor: String?,
        accessToken: String
    ) async throws(TimelineError) -> TimelinePage {
        await wait()
        let entries = Self.entries(for: feed, in: cyphers)
        let start = cursor.map { Int($0) ?? -1 } ?? 0
        guard entries.indices.contains(start) || start == entries.count else {
            throw .unexpected("invalid cursor: \(cursor ?? "")")
        }
        let end = min(start + Self.pageSize, entries.count)
        return TimelinePage(
            entries: Array(entries[start..<end]),
            nextCursor: end < entries.count ? String(end) : nil
        )
    }

    public func cypher(id: Cypher.ID, accessToken: String) async throws(TimelineError) -> Cypher {
        await wait()
        guard let cypher = cyphers.first(where: { $0.id == id }) else { throw .notFound }
        return cypher
    }

    private func wait() async {
        try? await Task.sleep(for: latency)
    }
}

extension StubTimelineRepository {
    private static let rappers = [
        Rapper(id: "rapper-0", name: "dozens", nickname: "Dozens"),
        Rapper(id: "rapper-1", name: "mc_kenta", nickname: "MC Kenta"),
        Rapper(id: "rapper-2", name: "lil_yuki", nickname: "Lil Yuki"),
        Rapper(id: "rapper-3", name: "big_sho", nickname: "Big Sho"),
        Rapper(id: "rapper-4", name: "rina_flow", nickname: "Rina Flow"),
    ]

    private static let followingRapperIDs: Set<Rapper.ID> = ["rapper-1", "rapper-3", "rapper-4"]

    private static let beats = [
        Beat(id: "beat-0", title: "Midnight Tokyo", artistName: "Dozens Beats"),
        Beat(id: "beat-1", title: "Boom Bap Classic", artistName: "Dozens Beats"),
        Beat(id: "beat-2", title: "Trap Rain", artistName: "Dozens Beats"),
    ]

    private static let videoURL = Bundle.module.url(forResource: "stub-rap", withExtension: "mp4")!

    private static func makeCyphers(now: Date) -> [Cypher] {
        (0..<12).map { (c: Int) -> Cypher in
            let id = "cypher-\(c)"
            let raps = (0..<(2 + c % 3)).map { (r: Int) -> Rap in makeRap(cypher: c, index: r, cypherID: id, now: now) }
            return Cypher(id: id, beat: beats[c % beats.count], raps: raps)
        }
    }

    private static func makeRap(cypher c: Int, index r: Int, cypherID: Cypher.ID, now: Date) -> Rap {
        let statistics = RapStatistics(
            views: (c * 37 + r * 11) % 500 + 3,
            likes: (c * 13 + r * 7) % 120,
            comments: (c * 5 + r * 3) % 30,
            hookups: r
        )
        let minutesAgo = Double(c * 3 + r) * 47
        return Rap(
            id: "rap-\(c)-\(r)",
            cypherID: cypherID,
            rapper: rappers[(c + r) % rappers.count],
            videoURL: videoURL,
            statistics: statistics,
            createdAt: now.addingTimeInterval(-minutesAgo * 60)
        )
    }

    private static func entries(for feed: TimelineFeed, in cyphers: [Cypher]) -> [TimelineEntry] {
        let all = cyphers.flatMap { cypher in
            cypher.raps.enumerated().map { index, rap in
                index == 0
                    ? TimelineEntry(kind: .cypher, message: "\(rap.rapper.name) started a cypher", rap: rap, beat: cypher.beat)
                    : TimelineEntry(kind: .rap, message: "\(rap.rapper.name) hooked up", rap: rap, beat: cypher.beat)
            }
        }
        switch feed {
        case .following:
            return all.filter { followingRapperIDs.contains($0.rap.rapper.id) }
                .sorted { $0.rap.createdAt > $1.rap.createdAt }
        case .featured:
            return all.sorted { $0.rap.statistics.views > $1.rap.statistics.views }
        case .newArrivals:
            return all.sorted { $0.rap.createdAt > $1.rap.createdAt }
        }
    }
}
