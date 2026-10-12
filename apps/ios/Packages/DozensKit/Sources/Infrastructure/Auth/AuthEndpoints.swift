import Domain
import Foundation

enum AuthEndpoints {
    struct EmailBody: Encodable {
        let email: String
    }

    struct ConfirmSignUpBody: Encodable {
        let email: String
        let code: String
    }

    struct ConfirmSignInBody: Encodable {
        let session: String
        let email: String
        let code: String
    }

    struct SignUpResponse: Decodable {
        let email: String
    }

    struct SignInResponse: Decodable {
        let email: String
        let session: String
    }

    struct TokensResponse: Decodable {
        let accessToken: String
        let refreshToken: String
        let idToken: String
    }

    struct UserResponse: Decodable {
        struct UserBody: Decodable {
            let id: UUID
            let name: String
        }

        let user: UserBody
    }

    static func signUp(email: Email) -> Endpoint<SignUpResponse> {
        Endpoint(method: .post, path: "sign_up", body: EmailBody(email: email.rawValue))
    }

    static func confirmSignUp(email: Email, code: OneTimeCode) -> Endpoint<TokensResponse> {
        Endpoint(
            method: .post,
            path: "confirm_sign_up",
            body: ConfirmSignUpBody(email: email.rawValue, code: code.rawValue)
        )
    }

    static func signIn(email: Email) -> Endpoint<SignInResponse> {
        Endpoint(method: .post, path: "sign_in", body: EmailBody(email: email.rawValue))
    }

    static func confirmSignIn(challenge: SignInChallenge, code: OneTimeCode) -> Endpoint<TokensResponse> {
        Endpoint(
            method: .post,
            path: "confirm_sign_in",
            body: ConfirmSignInBody(
                session: challenge.session,
                email: challenge.email.rawValue,
                code: code.rawValue
            )
        )
    }

    static func signOut(accessToken: String) -> Endpoint<EmptyResponse> {
        Endpoint(method: .delete, path: "sign_out", accessToken: accessToken)
    }

    static func user(accessToken: String) -> Endpoint<UserResponse> {
        Endpoint(method: .get, path: "user", accessToken: accessToken)
    }
}

extension AuthTokens {
    init(_ response: AuthEndpoints.TokensResponse) {
        self.init(
            accessToken: response.accessToken,
            refreshToken: response.refreshToken,
            idToken: response.idToken
        )
    }
}
