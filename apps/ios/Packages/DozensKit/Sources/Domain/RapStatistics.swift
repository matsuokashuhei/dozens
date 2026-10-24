public struct RapStatistics: Hashable, Sendable {
    public var views: Int
    public var likes: Int
    public var comments: Int
    public var hookups: Int

    public init(views: Int = 0, likes: Int = 0, comments: Int = 0, hookups: Int = 0) {
        self.views = views
        self.likes = likes
        self.comments = comments
        self.hookups = hookups
    }
}
