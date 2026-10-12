import DesignSystem
import SwiftUI

struct SubmitButton<Label: View>: View {
    let isSubmitting: Bool
    let isEnabled: Bool
    let action: () async -> Void
    @ViewBuilder let label: Label

    var body: some View {
        Button {
            Task { await action() }
        } label: {
            ZStack {
                label.opacity(isSubmitting ? 0 : 1)
                if isSubmitting { ProgressView() }
            }
            .font(.dozens(.headingSmall))
            .frame(maxWidth: .infinity)
            .padding(.vertical, 6)
        }
        .dozensProminentButtonStyle()
        .disabled(!isEnabled)
    }
}
