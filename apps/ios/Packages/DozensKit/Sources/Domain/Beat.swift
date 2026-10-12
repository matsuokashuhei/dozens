public struct Beat: Identifiable, Hashable, Sendable {
    public let id: String
    public var title: String
    public var artistName: String?
    public var parts: [BeatPart]

    public init(id: String, title: String, artistName: String? = nil, parts: [BeatPart] = []) {
        self.id = id
        self.title = title
        self.artistName = artistName
        self.parts = parts
    }
}
