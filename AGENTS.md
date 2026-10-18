# コーディング規約

コードのあるべき形を定める。

## 命名

エラー(enum のバリアント)は「対象(名詞)を先に、状態・述語を後に置く」並びで命名する。

- ✅ `UserNotFound`, `UserAlreadyExists`, `SelfFollowNotAllowed`, `InvalidEmail`
- ❌ `NotFoundUser`(名詞が後) / `FollowSelf`(動詞 + 目的語の命令形)

- 状態語は短く: `Invalid`, `Expired`, `Mismatch`, `NotFound`, `AlreadyExists` など
- 説明が必要なときは `NotAllowed` のように付ける(`SelfFollowNotAllowed`)
- `#[error("...")]` のメッセージは小文字で始める(例: `"self follow is not allowed"`)

## 参照

同名のアイテムを複数参照する場合:

- **参照元と参照先の距離が遠い方を、パッケージ名付きで呼ぶ**(例: `user_id::UserId` / `crate::domain::model::user_id::UserId`)
- 近い方は bare の `use` でよい
- `use ... as ...` による別名は定義しない

例: `infrastructure::repository` のファイルから `domain::model::user_id::UserId`(遠い層)と `user_repository::UserId`(近い兄弟)を参照する場合、ドメイン側は `user_id::UserId` と書き、兄弟側は `UserId` とする。

## 関数設計

単一責任の原則にしたがう。

- 1 関数 = 1 責務。関数名に `and` が必要な時点で責務が混在している(例: `init_and_migrate` はトレーシング初期化とマイグレーションを抱えていた)。分割して呼び出し側で合成する
- 一方で、1 行の呼び出しを包むだけの関数は定義しない(例: `tracing_subscriber::fmt::try_init()` だけを呼ぶ `init_tracing()`)。そのまま書けるものを関数化すると、定義・命名・参照ジャンプのコストだけが増える
- 関数に切り出すのは「実質的な処理単位」または「意味のある重複の排除」になる場合のみ(例: DB 接続 + `OnceCell` による 1 回限りのマイグレーション適用 `migrate()` は関数にする価値がある)

## ロギング

- 重要でない情報はログに出さない。「〜を開始した」のような事実だけの進行ログ(例: `info!("Setting up test environment")`)はノイズなので書かない・残さない
- ログは失敗の診断や状態の判断に必要な情報がある場合に限る

# 開発手順

作業のやり方を定める。

## テスト実行

`apps/api` で `cargo test` する。DB テスト(repository 層)はテスト用 Postgres と `.env.test` の環境変数が必要。

```sh
docker compose -f apps/compose.yml up -d postgres_test  # localhost:5433 / dozens_test
cd apps/api
set -a && source ../../.env.test && set +a
cargo test
```

- Cognito 実呼び出しが必要なテストは `#[ignore = "requires AWS Cognito"]` 付きで、ローカルでは実行しない。これらだけが `set_up_with_cognito()` / `tear_down_with_cognito()` を使う
- DB テストは `set_up()` / `tear_down()`(DB のみ)を使い Cognito API にはアクセスしないが、`issuer()` が `AWS_REGION` / `AWS_COGNITO_USER_POOL_ID` を参照するテスト(`local_token_decoder`, `test_get_user_by_sub_succeeds`)は `.env.test` の読み込みが必要

# プロジェクト管理

コードでも作業でもない、このリポジトリの管理に関する情報。

## エージェントスキル

プロジェクトレベルで管理する。
