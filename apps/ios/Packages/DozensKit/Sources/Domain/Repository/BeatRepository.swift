public protocol BeatRepository: Sendable {
    func beats(accessToken: String) async throws(RecordingError) -> [Beat]
    func beat(id: Beat.ID, accessToken: String) async throws(RecordingError) -> Beat
}
