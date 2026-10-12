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

    /// `DOZENS_BACKEND=stub` でスタブ、それ以外は Info.plist の `DozensAPIBaseURL` を使う。
    public static func fromEnvironment(
        bundle: Bundle = .main,
        environment: [String: String] = ProcessInfo.processInfo.environment
    ) -> AppConfiguration {
        if environment["DOZENS_BACKEND"] == "stub" {
            return AppConfiguration(backend: .stub)
        }
        let raw = environment["DOZENS_API_BASE_URL"]
            ?? bundle.object(forInfoDictionaryKey: "DozensAPIBaseURL") as? String
        let url = raw.flatMap(URL.init(string:)) ?? URL(string: "http://localhost:3000")!
        return AppConfiguration(backend: .api(url))
    }
}
