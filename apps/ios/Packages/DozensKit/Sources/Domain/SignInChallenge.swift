/// Cognito の EMAIL_OTP チャレンジ。`sign_in` が返す session を `confirm_sign_in` に渡す。
public struct SignInChallenge: Hashable, Sendable {
    public let email: Email
    public let session: String

    public init(email: Email, session: String) {
        self.email = email
        self.session = session
    }
}
