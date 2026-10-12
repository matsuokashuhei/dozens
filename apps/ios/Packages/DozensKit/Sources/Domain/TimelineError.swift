public enum TimelineError: Error, Equatable, Sendable {
    case notFound
    case unauthorized
    case network
    case unexpected(String)
}
