import Application
import Domain
import Observation

@Observable
public final class TimelineModel {
    /// 変更すると表示中のエントリを破棄する。読み込みは View の `.task(id:)` が行う。
    public var feed: TimelineFeed = .following {
        didSet {
            guard feed != oldValue else { return }
            entries = []
            nextCursor = nil
            error = nil
            isLoaded = false
        }
    }

    public private(set) var entries: [TimelineEntry] = []
    /// 現在のフィードを一度でも読み終えたか（失敗を含む）。読み込み前の空表示と「0 件」を区別する。
    public private(set) var isLoaded = false
    public private(set) var error: TimelineError?

    private var nextCursor: String?
    private var isLoadingMore = false
    /// プレイヤーのフックアップボタンが押されたときに呼ばれる。
    public var onHookUp: ((Cypher, Rap) -> Void)?
    private let reader: TimelineReader

    public init(reader: TimelineReader) {
        self.reader = reader
    }

    public func reload() async {
        let feed = self.feed
        do {
            let page = try await reader.page(of: feed, after: nil)
            guard feed == self.feed else { return }
            entries = page.entries
            nextCursor = page.nextCursor
            error = nil
            isLoaded = true
        } catch {
            guard feed == self.feed else { return }
            self.error = error
            isLoaded = true
        }
    }

    /// `entry` が末尾なら次のページを追加する。失敗した場合は次にスクロールしたときに再試行される。
    public func loadMore(ifReached entry: TimelineEntry) async {
        guard entry.id == entries.last?.id, let cursor = nextCursor, !isLoadingMore else { return }
        let feed = self.feed
        isLoadingMore = true
        defer { isLoadingMore = false }
        guard let page = try? await reader.page(of: feed, after: cursor), feed == self.feed else { return }
        let known = Set(entries.map(\.id))
        entries += page.entries.filter { !known.contains($0.id) }
        nextCursor = page.nextCursor
    }

    public func playerModel(for entry: TimelineEntry) -> CypherPlayerModel {
        let player = CypherPlayerModel(cypherID: entry.rap.cypherID, startRapID: entry.rap.id, reader: reader)
        player.onHookUp = onHookUp
        return player
    }
}
