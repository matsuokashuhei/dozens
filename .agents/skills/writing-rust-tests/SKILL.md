---
name: writing-rust-tests
description: Use when adding, modifying, or reviewing Rust unit tests in this workspace, including tests for domain models, repositories, use cases, routers, and services.
---

# Rust テスト関数の命名・構成ルール

## 目的

テストの名前から「どのメソッドを、どのケースで検証したか」を確定できるようにし、失敗時に原因を絞り込めるようにする。

## ルール 1: 命名は `test_<テスト対象のメソッド名>_<テストケース>`

- `<テスト対象のメソッド名>` は検証するメソッド名そのものを書く。**型名・モジュール名を接頭辞として付けない**(テストは対象と同じファイルの `mod tests` にあるため、文脈で十分)。
- `<テストケース>` は検証内容の短い説明(英語・snake_case)。
- 成功ケースは `with_success` の形を使う。
- テスト対象が入力を拒否するエラー(バリデーション違反・一意制約違反など)は `rejects_<原因>` の形を使う。
- `NotFound` のように**「返す」結果として生じるエラー**は `with_<状態>` の形を使う(`with_not_found` / `with_user_not_found`)。

例:

| 判定 | 名前 | 理由 |
|---|---|---|
| ❌ | `test_user_relationship_new` | `UserRelationship::new` の成否のみ。型名が付き、テストケースが無い |
| ✅ | `test_new_with_success` | `new` + 成功ケース |
| ✅ | `test_new_rejects_self_follow` | `new` + 自己フォローを拒否するケース |
| ✅ | `test_create_user_relationship_rejects_duplicate` | `create_user_relationship` + 重複登録を拒否するケース |
| ✅ | `test_get_user_relationship_with_not_found` | `get_user_relationship` + NotFound を返すケース |

## ルール 2: 1 テスト関数 = 1 メソッド × 1 テストケース

- テスト関数内で検証するのは**1 つのメソッド**に対する**1 つのケース**のみ。複数メソッドを鎖状に呼んで検証しない。

| 判定 | 名前 | 理由 |
|---|---|---|
| ❌ | `test_user_relationship_crud` | create / get / list×2 / delete を 1 関数で検証している |

分割後の例(`UserRelationshipRepository` の場合):

- `test_create_user_relationship_with_success`
- `test_get_user_relationship_with_success`
- `test_list_user_relationships_by_follower_with_success`
- `test_list_user_relationships_by_followee_with_success`
- `test_delete_user_relationship_with_success`

- **分割による準備(fake データ作成や setup)の重複は許容する。** このリポジトリーの DB テストは `set_up()` / `tear_down()` が各テストで分離を保証する。準備が冗長になったらデータ作成をテスト支援関数・fixture に切り出すこと(分岐やケースが異なるテストは絶対に融合させない)。
- `#[tokio::test]` を要する async テスト、`#[test]` は sync テスト。既存の同種テストと一致させる。

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

Rust の構文規約 `#[test]` の設定方法や async テストの扱いは、既存の関連テストを現行実装として参照すること。このリポジトリーの例は `apps/api/src/infrastructure/repository/` 配下にある。
