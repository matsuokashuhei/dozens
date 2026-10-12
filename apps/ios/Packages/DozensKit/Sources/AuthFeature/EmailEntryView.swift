import DesignSystem
import SwiftUI

struct EmailEntryView: View {
    @Bindable var model: AuthFlowModel
    @FocusState private var focused: Bool

    var body: some View {
        VStack(alignment: .leading, spacing: 16) {
            Text("Enter your email address", bundle: .module)
                .font(.dozens(.headingLarge))
            Text("We'll send you a 6-digit code to sign in or create an account.", bundle: .module)
                .font(.dozens(.bodyRegular))
                .foregroundStyle(.secondary)
            TextField(text: $model.emailText) {
                Text(verbatim: "you@example.com")
            }
            #if !os(macOS)
            .keyboardType(.emailAddress)
            .textInputAutocapitalization(.never)
            #endif
            .textContentType(.emailAddress)
            .autocorrectionDisabled()
            .submitLabel(.next)
            .focused($focused)
            .onSubmit { Task { await model.submitEmail() } }
            .padding()
            .dozensGlassEffect(in: .rect(cornerRadius: 12))
            .accessibilityIdentifier("auth.email")
            Spacer()
            SubmitButton(isSubmitting: model.isSubmitting, isEnabled: model.canSubmitEmail) {
                await model.submitEmail()
            } label: {
                Text("Send Code", bundle: .module)
            }
            .accessibilityIdentifier("auth.email.submit")
        }
        .padding(24)
        .frame(maxWidth: .infinity, maxHeight: .infinity, alignment: .topLeading)
        .background(.dozensBackground)
        .onAppear { focused = true }
    }
}
