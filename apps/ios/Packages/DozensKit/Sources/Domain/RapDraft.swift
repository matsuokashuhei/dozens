import Foundation

/// 投稿前のラップ。`hookUpRapID` があればそのラップのサイファーへのフックアップとして投稿する。
public struct RapDraft: Hashable, Sendable {
    public var videoURL: URL
    public var beatPartID: BeatPart.ID
    public var hookUpRapID: Rap.ID?
    public var lyrics: String
    public var tags: [String]
    public var language: String

    public init(
        videoURL: URL,
        beatPartID: BeatPart.ID,
        hookUpRapID: Rap.ID? = nil,
        lyrics: String = "",
        tags: [String] = [],
        language: String = "ja"
    ) {
        self.videoURL = videoURL
        self.beatPartID = beatPartID
        self.hookUpRapID = hookUpRapID
        self.lyrics = lyrics
        self.tags = tags
        self.language = language
    }
}
