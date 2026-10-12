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
    public var codeText = "" {
        didSet {
            let sanitized = String(codeText.filter(\.isNumber).prefix(OneTimeCode.length))
            if sanitized != codeText { codeText = sanitized }
        }
    }

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
