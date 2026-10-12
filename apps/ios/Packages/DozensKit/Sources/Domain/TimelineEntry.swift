public struct TimelineEntry: Identifiable, Hashable, Sendable {
    public enum Kind: Hashable, Sendable {
        /// サイファーを始めた
        case cypher
        /// 既存のサイファーにラップを重ねた（フックアップ）
        case rap
    }

    public var kind: Kind
    public var message: String
    public var rap: Rap
    public var beat: Beat

    public var id: Rap.ID { rap.id }

    public init(kind: Kind, message: String, rap: Rap, beat: Beat) {
        self.kind = kind
        self.message = message
        self.rap = rap
        self.beat = beat
    }
}
