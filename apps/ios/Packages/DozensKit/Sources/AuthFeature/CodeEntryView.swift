import DesignSystem
import Domain
import SwiftUI

struct CodeEntryView: View {
    @Bindable var model: AuthFlowModel
    @FocusState private var focused: Bool

    var body: some View {
        VStack(alignment: .leading, spacing: 16) {
            Text("Enter the code", bundle: .module)
                .font(.dozens(.headingLarge))
            if let email = model.pending?.email {
                Text("We sent a 6-digit code to \(email.rawValue).", bundle: .module)
                    .font(.dozens(.bodyRegular))
                    .foregroundStyle(.secondary)
            }
            TextField(text: $model.codeText) {
                Text(verbatim: "000000")
            }
            #if !os(macOS)
            .keyboardType(.numberPad)
            #endif
            .textContentType(.oneTimeCode)
            .font(.system(.title, design: .monospaced))
            .focused($focused)
            .padding()
            .dozensGlassEffect(in: .rect(cornerRadius: 12))
            .accessibilityIdentifier("auth.code")
            .onChange(of: model.codeText) { _, newValue in
                if newValue.count == OneTimeCode.length {
                    Task { await model.submitCode() }
                }
            }
            Button {
                Task { await model.resendCode() }
            } label: {
                Text("Resend code", bundle: .module)
                    .font(.dozens(.bodyRegular))
            }
            .disabled(model.isSubmitting)
            Spacer()
            SubmitButton(isSubmitting: model.isSubmitting, isEnabled: model.canSubmitCode) {
                await model.submitCode()
            } label: {
                Text("Continue", bundle: .module)
            }
            .accessibilityIdentifier("auth.code.submit")
        }
        .padding(24)
        .frame(maxWidth: .infinity, maxHeight: .infinity, alignment: .topLeading)
        .background(.dozensBackground)
        .onAppear { focused = true }
    }
}
