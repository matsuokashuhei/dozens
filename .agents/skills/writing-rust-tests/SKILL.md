---
name: writing-rust-tests
description: Use when adding, modifying, or reviewing Rust unit tests in this workspace, including tests for domain models, repositories, use cases, routers, and services.
---

# Rust テスト関数の命名・構成ルール

## 目的

テストの名前から「どのメソッドを、どのケースで検証したか」を確定できるようにし、失敗時に原因を絞り込めるようにする。

## ルール 1: 命名は `test_<テスト対象のメソッド名>_<テストケース>`

- `<テスト対象のメソッド名>` は検証するメソッド名そのものを書く。**型名・モジュール名を接頭辞として付けない**(テストは対象と同じファイルの `mod tests` にあるため、文脈で十分)。
- `<テストケース>` は検証内容の短い説明(英語・snake_case)。成否を動詞で先に述べる形に統一し、`succeeds` / `fails` を対にする。
- 正常系(エラーを返さない)ケースは `succeeds` 系を使う。
  - 無条件の成功は `succeeds` で終える(`test_new_succeeds`)。
  - 前提条件がある成功は `succeeds_when_<条件>`(主語 + 述語の節。`succeeds_when_user_already_exists`)。
  - 結果や状態が名詞句で述べたい場合は `succeeds_with_<名詞句>`(`succeeds_with_empty`)。
- 失敗系(`Err` や panic を検証する)ケースは `fails_when_<失敗を引き起こす条件>` の形を使う。
  - `<条件>` には失敗の**理由・エラー名ではなく発生条件**を、主語 + 述語の節で書く(`fails_when_follower_equals_followee`)。エラーバリアントはアサーション側で担保するため、名前に含めなくてよい。
  - 接続詞は `when` を使う。`where` は状況・場所を導く語で条件提示には不自然。`with_failure_where_...` のように従属表現を重ねない。
  - バリデーション違反・一意制約違反・`NotFound` のような「返す」エラーも、失敗系であることに変わりはないので `rejects_` / `with_<エラー>` ではなく `fails_when_<条件>` に統一する。
  - 同一条件でも成否が分かれる場合は `succeeds_when_<条件>` と `fails_when_<条件>` が対になる(例: `request_sign_up` は失敗し、`sign_up` は成功する)。

例:

| 判定 | 名前 | 理由 |
|---|---|---|
| ❌ | `test_user_relationship_new` | `UserRelationship::new` の成否のみ。型名が付き、テストケースが無い |
| ❌ | `test_new_with_failure_where_self_follow_is_not_allowed` | `where` 節が失敗条件ではなくエラー名を指し、従属表現も重複している |
| ❌ | `test_confirm_sign_up_with_code_mismatch` | 失敗系なのに `with_<エラー>`。`fails_when_code_does_not_match` とする |
| ❌ | `test_new_with_success` | `with_success` は旧形式。`succeeds` とする |
| ✅ | `test_new_succeeds` | `new` + 無条件の成功ケース |
| ✅ | `test_new_fails_when_follower_equals_followee` | `new` + 失敗条件(同一 ID を渡す) |
| ✅ | `test_create_user_relationship_fails_when_relationship_already_exists` | `create_user_relationship` + 重複登録で失敗するケース |
| ✅ | `test_get_user_relationship_fails_when_relationship_does_not_exist` | `get_user_relationship` + NotFound となる失敗条件 |
| ✅ | `test_sign_up_succeeds_when_user_already_exists` | 既存ユーザーでもエラーにならない条件付きの正常系 |
| ✅ | `test_list_user_relationships_by_follower_succeeds_with_empty` | 空リストを返す正常系 |

## ルール 2: 1 テスト関数 = 1 メソッド × 1 テストケース

- テスト関数内で検証するのは**1 つのメソッド**に対する**1 つのケース**のみ。複数メソッドを鎖状に呼んで検証しない。

| 判定 | 名前 | 理由 |
|---|---|---|
| ❌ | `test_user_relationship_crud` | create / get / list×2 / delete を 1 関数で検証している |

分割後の例(`UserRelationshipRepository` の場合):

- `test_create_user_relationship_succeeds`
- `test_get_user_relationship_succeeds`
- `test_list_user_relationships_by_follower_succeeds`
- `test_list_user_relationships_by_followee_succeeds`
- `test_delete_user_relationship_succeeds`

- **分割による準備(fake データ作成や setup)の重複は許容する。** このリポジトリーの DB テストは `set_up()` / `tear_down()` が各テストで分離を保証する。準備が冗長になったらデータ作成をテスト支援関数・fixture に切り出すこと(分岐やケースが異なるテストは絶対に融合させない)。
- `#[tokio::test]` を要する async テスト、`#[test]` は sync テスト。既存の同種テストと一致させる。

## ルール 3: 正常系の入力値は `fake` クレートで生成する

- 値そのものに意味がなく「有効であること」だけが必要な入力(メール・名前など)はハードコードせず faker で生成する。`"A@B.com"` のような具体値は「その値でないと通らないのでは」という読み手の疑念を生む。
- 対応する faker を使う: メール → `faker::internet::raw::FreeEmail`、名前 → `faker::name::raw::Name`、単語 → `faker::lorem::raw::Word`(いずれも `locales::EN`)。
- 失敗条件であること自体が意味を持つ無効値(`"invalid-email"`、誤ったコード `"123456"` など)は faker にせず、条件が一目で分かるリテラルのままにする。
- 外部サービスに実在が必要な値(Cognito の `TEST_EMAILS` など)も例外としてリテラルを許容する。

例:

| 判定 | コード | 理由 |
|---|---|---|
| ❌ | `Email::new("A@B.com")` | 有効な値なら何でもよいのに具体値を固定している |
| ✅ | `let raw: String = FreeEmail(EN).fake(); Email::new(&raw)` | 有効なメールを faker で生成 |
| ✅ | `Email::new("invalid-email")` | 無効であること自体が条件。リテラルで明示 |

## リファクタリング時の注意

- 改名・分割・構成の見直しをしても、既存のテストケースやアサーションを削除・弱体化しない。テスト数と検証内容は維持か増強のみ。
- すでに規約に合っているテスト・ヘルパー・同期ガードは変更しない。規約違反の箇所だけを直す。
- DB を共有するテスト同士の直列化(`DB_LOCK` など)は維持する。`set_up()` / `tear_down()` が共通テーブルを消すため、並列実行すると不安定になる。
- エラーケースは `assert!(matches!(result, Err(ErrorType::Variant)))` のようにバリアントまで検証する。`is_err()` だけで終わらせない。

## 検証コマンド

cargo などの Rust コマンドはホストでは直接実行しない。`apps/compose.yml` の docker compose に定義された `api` サービス(Rust コンテナ)で実行する。コンテナからテスト DB には `postgres_test:5432` で到達するため `DATABASE_URL` を指定する。

```sh
# テスト DB の起動(リポジトリーのルートで)
docker compose -f apps/compose.yml up -d --wait postgres_test

# api コンテナのビルド(初回のみ)
docker compose -f apps/compose.yml build api

# cargo の実行例(--rm で使い捨てコンテナ)
COMPOSE="docker compose -f apps/compose.yml"
$COMPOSE run --rm api cargo fmt --all
$COMPOSE run --rm api cargo clippy --all-targets --all-features -- -Dwarnings
$COMPOSE run --rm \
  --env-from-file .env.test \
  -e GITHUB_ACTIONS=true \
  -e DATABASE_URL=postgresql://postgres:postgres@postgres_test:5432/dozens_test \
  api cargo test --all-features
```

- `--env-from-file .env.test` は `AWS_REGION` / `AWS_COGNITO_*` をコンテナに渡すため。
- `GITHUB_ACTIONS=true` は Cognito ユーザーの後片付けをスキップするローカル実行用の指定。
- `DATABASE_URL` は `test_support::build_db_connection` が読む。未指定時はホスト実行用の `localhost:5433` が使われる。
- リンク時に OOM(signal 9)になったら `cargo test --all-features -j 2` でリトライする。

## よくある言い訳

| 言い訳 | 現実 |
|---|---|
| 「CRUD だから 1 関数の流れの方が設計を表現する」 | 失敗時にどのメソッドが壊れたか不明になる。失敗特定が名前と結果で完結するのが目的 |
| 「型名を付けないと同名メソッドで検索しづらい」 | モジュール(`mod tests`)が対象を特定する。命名での重複は冗長 |
| 「この特徴テスト 1 個で十分」 | 1 関数に 1 メソッド × 1 ケースを割り当てるのが例外として認められる唯一の形ではない |
| 「setup が縁こそ重複するが融合した方が早い」 | ヘルパー化で重複削減。テストの融合で解決しない |

## Red Flags

- テスト名に `_crud`, `_flow`, `_end_to_end` などの接尾辞が付く → 分割する
- 名前が `test_` + メソッド名だけで終わる(ケース記述がない) → ケースを付ける
- 名前に型名(`user_relationship_`, `user_`)が付く → 取り除いてメソッド名から始める
- 失敗系なのに `with_<エラー>` / `rejects_<原因>` / `with_failure_where_` が使われている → `fails_when_<失敗条件>` に直す
- 正常系なのに `with_success` / `with_<状態>` が使われている → `succeeds` / `succeeds_when_<条件>` / `succeeds_with_<名詞句>` に直す
- `fails_when_` / `succeeds_when_` の後が名詞句だけで条件(主語 + 述語の節)になっていない → `fails_when_code_does_not_match` のような節に直す
- 有効でさえあればよい入力値(メール・名前)をリテラルで書いている → `FreeEmail` / `Name` などの faker に置き換える

Rust の構文規約 `#[test]` の設定方法や async テストの扱いは、既存の関連テストを現行実装として参照すること。このリポジトリーの例は `apps/api/src/infrastructure/repository/` 配下にある。
