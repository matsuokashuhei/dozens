import Domain

/// OTP 入力待ちの状態。既存ユーザーはサインイン、未登録ユーザーはサインアップのコードを待つ。
public enum PendingVerification: Hashable, Sendable {
    case signIn(SignInChallenge)
    case signUp(Email)

    public var email: Email {
        switch self {
        case .signIn(let challenge): challenge.email
        case .signUp(let email): email
        }
    }
}
