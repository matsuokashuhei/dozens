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

1. **HS256 ローカルデコーダ**(`infrastructure/service/local_token_decoder.rs`)
   - `LocalTokenDecoder` が `JwtDecoder<Claims>` を実装し、
     `jsonwebtoken::decode` + `DecodingKey::from_secret(SECRET)` で検証
   - `SECRET` はコード内にハードコード(開発専用、env不要)
   - `Validation` は Cognito 版と同じ設定(HS256, `iss`/`exp`/`sub` 必須、`aud` 無し)
   - `iss` は `CognitoIdentityProvider::issuer()` を使い、
     クレームはプロダクションの `Claims`(`sub`/`exp`/`iss`)そのもの

2. **開発用トークン発行バイナリ**(`src/bin/dev_token.rs`)
   - 実行ごとに UUID v7 のダミー `sub` を生成し、
     user + user_identity を作成
   - その `sub` で本番と同じ `Claims`(`{sub, iss, exp}`)の HS256 JWT を
     生成して `Authorization: Bearer <token>` を stdout に出力

### 起動時の切替

`api.rs` で `AUTH_MODE=local` のとき local decoder を使う:

```rust
let decoder: Decoder<Claims> = match env::var("AUTH_MODE").as_deref() {
    Ok("local") => {
        warn!("local auth mode; do not use in production");
        Arc::new(local_token_decoder::LocalTokenDecoder::new())
    }
    _ => Arc::new(identity_provider.build_token_decoder().await?),
};
```

## 環境変数

| 変数 | 値 | 備考 |
|---|---|---|
| `AUTH_MODE` | `cognito` (default) / `local` | デコーダ切替 |
| `AWS_REGION` / `AWS_COGNITO_USER_POOL_ID` | iss の構成値 | Cognito呼出しは不要、値だけ参照 |

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
