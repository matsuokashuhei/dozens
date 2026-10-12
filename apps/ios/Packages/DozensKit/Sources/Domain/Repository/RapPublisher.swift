public protocol RapPublisher: Sendable {
    func publish(_ draft: RapDraft, accessToken: String) async throws(RecordingError) -> Rap.ID
}
