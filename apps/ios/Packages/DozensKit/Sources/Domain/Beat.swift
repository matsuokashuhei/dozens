public struct Beat: Identifiable, Hashable, Sendable {
    public let id: String
    public var title: String
    public var artistName: String?

    public init(id: String, title: String, artistName: String? = nil) {
        self.id = id
        self.title = title
        self.artistName = artistName
    }
}
