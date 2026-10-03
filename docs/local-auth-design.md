# ローカル開発用認証(JWK非依存)仕様

## 背景

APIサーバはアクセストークンの検証に `RemoteJwksDecoder` を使い、
起動時に Cognito の `/.well-known/jwks.json` から公開鍵を取得する
(`CognitoIdentityProvider::build_token_decoder`)。
このためローカルで認証付きAPIを試すだけでも AWS Cognito が必要だった。

## 目的

- Cognito なしで「認証済み状態」を作り、認証付きAPI(例: GET /user)を
  ローカルで試せること
- サインアップ/サインインのフローはローカルでは不要。
  ダミーの subject を持つアクセストークンを発行するだけでよい
- トークンのデコード以外は既存コードをそのまま使うこと

## 方針

2つの部品だけを追加する。

1. **HS256 ローカルデコーダ**(`infrastructure/service/local_auth.rs`)
   - `axum_jwt_auth::LocalDecoder` を `DecodingKey::from_secret(SECRET)` で構築
   - `SECRET` はコード内にハードコード(開発専用、env不要)
   - `iss` は `CognitoIdentityProvider::issuer()`、
     `aud` は `AWS_COGNITO_USER_POOL_CLIENT_ID` と同じ値を使う
     → クレームの形は本番トークンと同じで、検証ロジックも同じ経路を通る

2. **開発用トークン発行バイナリ**(`src/bin/dev_token.rs`)
   - `LOCAL_AUTH_SUB`(省略時は固定UUID)を `get_user_by_sub` で検索し、
     無ければ user + user_identity を作成(冪等)
   - その `sub` で HS256 JWT(`{sub, iss, aud, exp, token_use:"access"}`)を
     生成して `Authorization: Bearer <token>` を stdout に出力

### 起動時の切替

`api.rs` で `AUTH_MODE=local` のとき local decoder を使う:

```rust
let decoder: Decoder<Claims> = match env::var("AUTH_MODE").as_deref() {
    Ok("local") => {
        warn!("local auth mode; do not use in production");
        Arc::new(local_auth::build_decoder()?)
    }
    _ => Arc::new(identity_provider.build_token_decoder().await?),
};
```

## 環境変数

| 変数 | 値 | 備考 |
|---|---|---|
| `AUTH_MODE` | `cognito` (default) / `local` | デコーダ切替 |
| `LOCAL_AUTH_SUB` | デフォルト `00000000-...-000000000000` | dev_token が使うダミーsubject |
| `AWS_REGION` / `AWS_COGNITO_USER_POOL_ID` / `AWS_COGNITO_USER_POOL_CLIENT_ID` | iss/aud の構成値 | Cognito呼出しは不要、値だけ参照 |

## 使い方

```sh
docker compose -f apps/compose.yml up -d postgres
cd apps/api && cargo run --bin toasty -- migration apply
AUTH_MODE=local cargo run --bin api          # AWS_* は .env.test 等で設定
cargo run --bin dev_token                    # => Authorization: Bearer ...
curl -H "Authorization: Bearer <token>" localhost:3000/user
```

## セキュリティ上の注意

- `AUTH_MODE=local` 選択時は起動ログに warn を出す
- `SECRET` は開発専用のハードコード値。本番で `AUTH_MODE=local` を
  設定しないことが前提(deploy 設定では `AUTH_MODE` を定義しない)
