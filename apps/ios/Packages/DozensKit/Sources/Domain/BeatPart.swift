import Foundation

/// ビートの一区間。録音はパート単位で行い、パートの長さが録画の上限になる。
public struct BeatPart: Identifiable, Hashable, Sendable {
    public let id: String
    public var name: String
    public var audioURL: URL
    public var duration: Duration

    public init(id: String, name: String, audioURL: URL, duration: Duration) {
        self.id = id
        self.name = name
        self.audioURL = audioURL
        self.duration = duration
    }
}
