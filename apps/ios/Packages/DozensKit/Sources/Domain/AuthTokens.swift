public struct AuthTokens: Hashable, Sendable, Codable {
    public let accessToken: String
    public let refreshToken: String
    public let idToken: String

    public init(accessToken: String, refreshToken: String, idToken: String) {
        self.accessToken = accessToken
        self.refreshToken = refreshToken
        self.idToken = idToken
    }
}
