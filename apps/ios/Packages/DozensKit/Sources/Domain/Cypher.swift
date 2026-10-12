/// 1 つのビートに複数のラッパーが順番にラップを重ねていくスレッド。`raps` は投稿順。
public struct Cypher: Identifiable, Hashable, Sendable {
    public let id: String
    public var beat: Beat
    public var raps: [Rap]

    public init(id: String, beat: Beat, raps: [Rap]) {
        self.id = id
        self.beat = beat
        self.raps = raps
    }
}
