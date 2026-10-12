import Foundation

public enum HTTPMethod: String, Sendable {
    case get = "GET"
    case post = "POST"
    case put = "PUT"
    case patch = "PATCH"
    case delete = "DELETE"
}

public struct EmptyResponse: Decodable, Sendable {}

public struct Endpoint<Response: Decodable & Sendable>: Sendable {
    public let method: HTTPMethod
    public let path: String
    public let body: Data?
    public let accessToken: String?

    public init(method: HTTPMethod, path: String, accessToken: String? = nil) {
        self.method = method
        self.path = path
        self.body = nil
        self.accessToken = accessToken
    }

    public init(
        method: HTTPMethod,
        path: String,
        body: some Encodable,
        accessToken: String? = nil
    ) {
        self.method = method
        self.path = path
        // Encodable な DTO の JSON 化は失敗しない前提
        self.body = try? JSONEncoder.api.encode(body)
        self.accessToken = accessToken
    }
}
