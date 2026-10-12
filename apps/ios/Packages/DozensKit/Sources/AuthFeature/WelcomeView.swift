import DesignSystem
import SwiftUI

struct WelcomeView: View {
    let onStart: () -> Void

    var body: some View {
        VStack(spacing: 24) {
            Spacer()
            DozensLogo()
                .frame(maxWidth: 200)
            Text("Rap with the world.", bundle: .module)
                .font(.dozens(.headingSmall))
                .foregroundStyle(.secondary)
            Spacer()
            Button(action: onStart) {
                Text("Get Started", bundle: .module)
                    .font(.dozens(.headingSmall))
                    .frame(maxWidth: .infinity)
                    .padding(.vertical, 6)
            }
            .buttonStyle(.glassProminent)
            .accessibilityIdentifier("auth.start")
        }
        .padding(24)
        .frame(maxWidth: .infinity, maxHeight: .infinity)
        .background(.dozensBackground)
    }
}

#Preview {
    WelcomeView(onStart: {})
        .preferredColorScheme(.dark)
}
