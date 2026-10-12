public enum APIError: Error, Equatable, Sendable {
    case transport(String)
    case status(code: Int, message: String)
    case decoding(String)
}
