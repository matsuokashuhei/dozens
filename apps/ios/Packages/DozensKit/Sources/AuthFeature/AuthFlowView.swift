import DesignSystem
import SwiftUI

public struct AuthFlowView: View {
    @Bindable private var model: AuthFlowModel

    public init(model: AuthFlowModel) {
        self.model = model
    }

    public var body: some View {
        NavigationStack(path: $model.path) {
            WelcomeView(onStart: model.begin)
                .navigationDestination(for: AuthFlowModel.Route.self) { route in
                    switch route {
                    case .email: EmailEntryView(model: model)
                    case .code: CodeEntryView(model: model)
                    }
                }
        }
        .alert(
            Text("Error", bundle: .module),
            isPresented: Binding(
                get: { model.error != nil },
                set: { if !$0 { model.error = nil } }
            ),
            presenting: model.error
        ) { _ in
            Button(role: .cancel) {} label: { Text("OK", bundle: .module) }
        } message: { error in
            Text(error.message)
        }
    }
}
