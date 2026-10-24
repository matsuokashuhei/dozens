import Foundation

/// サイファーに投稿された 1 本のラップ。`videoURL` はビートを合成済みの動画。
public struct Rap: Identifiable, Hashable, Sendable {
    public let id: String
    public var cypherID: Cypher.ID
    public var rapper: Rapper
    public var videoURL: URL
    public var thumbnailURL: URL?
    public var statistics: RapStatistics
    public var createdAt: Date

    public init(
        id: String,
        cypherID: Cypher.ID,
        rapper: Rapper,
        videoURL: URL,
        thumbnailURL: URL? = nil,
        statistics: RapStatistics = RapStatistics(),
        createdAt: Date
    ) {
        self.id = id
        self.cypherID = cypherID
        self.rapper = rapper
        self.videoURL = videoURL
        self.thumbnailURL = thumbnailURL
        self.statistics = statistics
        self.createdAt = createdAt
    }
}
