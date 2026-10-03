# ローカル開発用認証(JWK非依存)仕様案

## 背景

現状、APIサーバはアクセストークンの検証に `RemoteJwksDecoder` を使い、
起動時に Cognito の `/.well-known/jwks.json` から公開鍵を取得している
(`CognitoIdentityProvider::build_token_decoder`)。
このため、ローカルで認証付きAPIを試すだけでも AWS Cognito が必要になる。

## 目的

- Cognito なしで「認証済み状態」を作り、認証付きAPI(例: GET /users/me)を
  ローカルで試せること
- サインアップ/サインインのフローはローカルでは不要。
  **ダミーの subject を持つアクセストークンを発行する**ことだけでよい
- middleware層(`TokenAuthenticator`/`UserAuthenticator`)は無変更にすること

## 方針

2つの部品だけを追加する。

1. **HS256 ローカルデコーダ**(`LocalTokenDecoder`)
   - `JwtDecoder<Claims>` を実装し、`jsonwebtoken::decode` +
     `DecodingKey::from_secret(LOCAL_AUTH_SECRET)` で検証する
   - `Validation` は本番と同じ設定(HS256、 `iss`/`exp`/`sub` 必須、`aud` 無し)
   - JWK 取得不要、署名検証は残るので本番と同じコードパスを通る

2. **開発用トークン発行バイナリ**(例 `src/bin/dev_token.rs`)
   - ダミー `sub`(固定UUID or 引数)を持つユーザを DB に用意する
     - `users` レコード + `user_identities`(iss, sub) レコードを作成
       (既存の `UserRepository`/`UserIdentityRepository` を使う)
     - `AuthenticateUserUsecase` は `sub` で `user_identities` を join して
       ユーザを引くため、**この行がないと 404 になる**
   - その `sub` で HS256 JWT(`{sub, iss, exp, token_use:"access"}`)を生成し
     stdout に出力する
   - 使い方例:
     ```
     cargo run --bin dev_token
     # => Authorization: Bearer eyJhbGciOi... をそのまま出力
     ```

### 起動時の切替

`api.rs` で環境変数によりデコーダを切り替える。

```rust
let decoder = match env::var("AUTH_MODE").as_deref() {
    Ok("local") => build_local_decoder()?,        // HS256
    _ => identity_provider.build_token_decoder().await?, // 既存 RS256/JWKS
};
```

`AUTH_MODE=local` のとき `aws_config`/`AWS_*` の読み込み自体をスキップし、
Cognito なしで起動できるようにする。
(サインアップ/サインイン系ルートは Cognito に依存するため、local モードでは
認証付きルートのみ検証対象とする。ルート登録自体はそのままでよい)

## 環境変数

| 変数 | 値 | 備考 |
|---|---|---|
| `AUTH_MODE` | `cognito` (default) / `local` | デコーダ切替 |
| `LOCAL_AUTH_SECRET` | 任意の文字列 | `local` 時必須。HS256署名鍵。APIとdev_tokenで共有 |
| `LOCAL_AUTH_ISSUER` | デフォルト `dozens-local` | `iss` に使う値 |
| `LOCAL_AUTH_SUB` | デフォルト `00000000-0000-0000-0000-000000000000` | dev_token が使うダミーsubject |

## 検討した代替案

| 案 | 却下理由 |
|---|---|
| 署名検証を完全にスキップするデコーダ | `exp`/claim検証が本番と乖離し、誤って有効化すると危険 |
| RS256 + ローカルRSA鍵ペア | Cognito形式に近いが鍵生成・配置が煩雑。共通鍵で十分 |
| Localstack / cognito-local | 目的(認証状態の作成)に対してセットアップが重い |

## セキュリティ上の注意

- `AUTH_MODE=local` 選択時は起動ログに
  `warn!("local auth mode; do not use in production")` を出す
- `LOCAL_AUTH_SECRET` を本番環境の secrets/env に設定しない
  (deploy 設定では `AUTH_MODE` 自体を定義しない)

## 変更ファイル

- 新規: `src/infrastructure/service/local_token_decoder.rs`
  (`JwtDecoder<Claims>` 実装 + `encode`/`decode` ヘルパー)
- 新規: `src/bin/dev_token.rs` (ユーザ作成 + トークン出力)
- 変更: `src/bin/api.rs` (`AUTH_MODE` 分岐)
- `IdentityProvider` トレイト・middleware・usecase は **変更なし**
