import Domain

/// 保存済みのアクセストークンを付けてビートの取得とラップの投稿を行う。
public struct RecordingService: Sendable {
    private let beatRepository: any BeatRepository
    private let publisher: any RapPublisher
    private let tokenStore: any TokenStore

    public init(beatRepository: any BeatRepository, publisher: any RapPublisher, tokenStore: any TokenStore) {
        self.beatRepository = beatRepository
        self.publisher = publisher
        self.tokenStore = tokenStore
    }

    public func beats() async throws(RecordingError) -> [Beat] {
        try await beatRepository.beats(accessToken: accessToken())
    }

    public func beat(id: Beat.ID) async throws(RecordingError) -> Beat {
        try await beatRepository.beat(id: id, accessToken: accessToken())
    }

    public func publish(_ draft: RapDraft) async throws(RecordingError) -> Rap.ID {
        try await publisher.publish(draft, accessToken: accessToken())
    }

    private func accessToken() throws(RecordingError) -> String {
        guard let tokens = try? tokenStore.load() else { throw .unauthorized }
        return tokens.accessToken
    }
}
