import Foundation

public struct User: Identifiable, Hashable, Sendable, Codable {
    public let id: UUID
    public var name: String

    public init(id: UUID, name: String) {
        self.id = id
        self.name = name
    }
}
