import Foundation

public struct Rapper: Identifiable, Hashable, Sendable {
    public let id: String
    public var name: String
    public var nickname: String?
    public var pictureURL: URL?

    public init(id: String, name: String, nickname: String? = nil, pictureURL: URL? = nil) {
        self.id = id
        self.name = name
        self.nickname = nickname
        self.pictureURL = pictureURL
    }
}
