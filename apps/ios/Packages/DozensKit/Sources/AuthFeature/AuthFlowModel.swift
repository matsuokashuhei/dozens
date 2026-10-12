import Application
import Domain
import Observation

@Observable
public final class AuthFlowModel {
    public enum Route: Hashable, Sendable {
        case email
        case code
    }

    public var path: [Route] = []
    public var emailText = ""
    public var codeText = ""

    public var error: AuthError?
    public private(set) var pending: PendingVerification?
    public private(set) var isSubmitting = false

    private let authenticator: PasswordlessAuthenticator
    private let session: SessionStore

    public init(authenticator: PasswordlessAuthenticator, session: SessionStore) {
        self.authenticator = authenticator
        self.session = session
    }

    public var canSubmitEmail: Bool {
        !isSubmitting && (try? Email(emailText)) != nil
    }

    public var canSubmitCode: Bool {
        !isSubmitting && pending != nil && (try? OneTimeCode(codeText)) != nil
    }

    /// `codeText` を半角数字 `OneTimeCode.length` 桁に切り詰め、揃ったら送信する。
    /// TextField の binding の中で書き換えると表示が追従しないので、`onChange` から呼ぶ。
    public func codeTextDidChange() {
        let sanitized = String(codeText.filter { $0.isASCII && $0.isNumber }.prefix(OneTimeCode.length))
        guard sanitized == codeText else {
            codeText = sanitized
            return
        }
        if codeText.count == OneTimeCode.length {
            Task { await submitCode() }
        }
    }

    public func begin() {
        path = [.email]
    }

    public func submitEmail() async {
        guard canSubmitEmail, let email = try? Email(emailText) else { return }
        isSubmitting = true
        defer { isSubmitting = false }
        do {
            pending = try await authenticator.start(email: email)
            codeText = ""
            path.append(.code)
        } catch {
            self.error = error
        }
    }

    public func submitCode() async {
        guard canSubmitCode, let pending, let code = try? OneTimeCode(codeText) else { return }
        isSubmitting = true
        defer { isSubmitting = false }
        do {
            let tokens = try await authenticator.verify(pending, code: code)
            try await session.signIn(with: tokens)
        } catch {
            codeText = ""
            self.error = error
        }
    }

    public func resendCode() async {
        guard !isSubmitting, let email = pending?.email else { return }
        isSubmitting = true
        defer { isSubmitting = false }
        do {
            pending = try await authenticator.start(email: email)
            codeText = ""
        } catch {
            self.error = error
        }
    }
}
