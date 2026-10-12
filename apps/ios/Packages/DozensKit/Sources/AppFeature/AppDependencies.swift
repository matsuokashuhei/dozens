import Domain
import Infrastructure

struct AppDependencies: Sendable {
    let authRepository: any AuthRepository
    let tokenStore: any TokenStore
    /// API にタイムライン・投稿のエンドポイントができるまでは、API モードでもスタブを使う。
    let timelineRepository: any TimelineRepository = StubTimelineRepository()
    let beatRepository: any BeatRepository = StubBeatRepository()
    let rapPublisher: any RapPublisher = StubRapPublisher()

    init(configuration: AppConfiguration) {
        switch configuration.backend {
        case .api(let baseURL):
            authRepository = APIAuthRepository(client: APIClient(baseURL: baseURL))
            tokenStore = KeychainTokenStore()
        case .stub:
            authRepository = StubAuthRepository()
            tokenStore = KeychainTokenStore(service: "app.dozens.ios.auth.stub")
        }
    }
}
