import Foundation

public struct AppConfiguration: Sendable, Equatable {
    public enum Backend: Sendable, Equatable {
        /// `apps/api` に接続する
        case api(URL)
        /// API なしでモック実装を使う
        case stub
    }

    public var backend: Backend

    public init(backend: Backend) {
        self.backend = backend
    }

    /// `DOZENS_BACKEND=stub` でスタブ、それ以外は `DOZENS_API_BASE_URL`（デフォルト `http://localhost:3000`）に接続する。
    public static func fromEnvironment(
        _ environment: [String: String] = ProcessInfo.processInfo.environment
    ) -> AppConfiguration {
        if environment["DOZENS_BACKEND"] == "stub" {
            return AppConfiguration(backend: .stub)
        }
        let url = environment["DOZENS_API_BASE_URL"].flatMap(URL.init(string:))
            ?? URL(string: "http://localhost:3000")!
        return AppConfiguration(backend: .api(url))
    }
}
