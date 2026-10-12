import Application
import AuthFeature
import DesignSystem
import SwiftUI

public struct AppRootView: View {
    @State private var session: SessionStore
    private let dependencies: AppDependencies

    public init(configuration: AppConfiguration) {
        let dependencies = AppDependencies(configuration: configuration)
        self.dependencies = dependencies
        _session = State(
            initialValue: SessionStore(
                repository: dependencies.authRepository,
                tokenStore: dependencies.tokenStore
            )
        )
    }

    public var body: some View {
        Group {
            switch session.state {
            case .restoring:
                SplashView()
            case .signedOut:
                SignedOutView(dependencies: dependencies, session: session)
            case .signedIn(let user):
                MainTabView(user: user, session: session, dependencies: dependencies)
            }
        }
        .animation(.default, value: session.state)
        .preferredColorScheme(.dark)
        .tint(.dozensGreen)
        .task { await session.restore() }
    }
}

private struct SplashView: View {
    var body: some View {
        VStack(spacing: 24) {
            DozensLogo().frame(maxWidth: 200)
            ProgressView()
        }
        .frame(maxWidth: .infinity, maxHeight: .infinity)
        .background(.dozensBackground)
    }
}

/// サインアウトのたびにフローの状態を作り直すため、モデルはこの View が所有する。
private struct SignedOutView: View {
    @State private var model: AuthFlowModel

    init(dependencies: AppDependencies, session: SessionStore) {
        _model = State(
            initialValue: AuthFlowModel(
                authenticator: PasswordlessAuthenticator(repository: dependencies.authRepository),
                session: session
            )
        )
    }

    var body: some View {
        AuthFlowView(model: model)
    }
}

#Preview("Stub") {
    AppRootView(configuration: AppConfiguration(backend: .stub))
}
