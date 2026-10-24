public struct TimelinePage: Hashable, Sendable {
    public var entries: [TimelineEntry]
    /// 次のページを取得するためのカーソル。最後のページでは `nil`。
    public var nextCursor: String?

    public init(entries: [TimelineEntry], nextCursor: String?) {
        self.entries = entries
        self.nextCursor = nextCursor
    }
}
