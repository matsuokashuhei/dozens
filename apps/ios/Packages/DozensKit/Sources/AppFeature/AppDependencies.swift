import Domain
import Infrastructure

struct AppDependencies: Sendable {
    let authRepository: any AuthRepository
    let tokenStore: any TokenStore

    init(configuration: AppConfiguration) {
        switch configuration.backend {
        case .api(let baseURL):
            authRepository = APIAuthRepository(client: APIClient(baseURL: baseURL))
            tokenStore = KeychainTokenStore()
        case .stub:
            authRepository = StubAuthRepository()
            tokenStore = InMemoryTokenStore()
        }
    }
}
