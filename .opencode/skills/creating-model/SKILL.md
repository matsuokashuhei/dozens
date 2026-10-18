---
name: Creating Model
description: apps/api にテーブル、ドメインモデル、リポジトリー、その実装、マイグレーションを追加または変更するときに使う。値オブジェクトの型化や CRUD の実装が絡むときもこれを使う。
---

# Creating Model

## 目的

新しい概念を永続化し、アプリケーションから操作できるようにする。ドメインの知識はドメイン層の型で表現し、toasty や SQL といった永続化の詳細はインフラストラクチャー層に閉じ込める。

## ゴール(成果物)

| 層 | 場所 | 成果物 |
| --- | --- | --- |
| ドメイン | `apps/api/src/domain/model/` | 集約モデルと値オブジェクト |
| ドメイン | `apps/api/src/domain/repository/` | リポジトリーのトレイトと `RepositoryError` |
| インフラストラクチャー | `apps/api/src/infrastructure/repository/` | toasty の Record、`From` 変換、リポジトリー実装 |
| インフラストラクチャー | `apps/api/src/infrastructure/repository/mod.rs` | `models()` への Record 登録 |
| データベース | `apps/api/toasty/migrations/` | マイグレーション SQL(スナップショットと `history.toml` も更新) |

完了条件:

- リポジトリーが CRUD の振る舞いを持つ
- 実装が Record からドメインモデルへの `From` を持つ
- 意味のある値が primitive ではなく型になっている
- 振る舞いをテストで確認している

## 設計の原則

- **値は意味で型にする。** ID、Eメール、位置情報など、誤用や不正な値を防ぎたい値は newtype にする。不変条件はコンストラクターで保証する(`Email::new` が `Result` を返すように)。ドメインの newtype は toasty に依存しない。toasty の derive(`Model` と `Embed`)はすべてインフラストラクチャー層だけに置く。

```rust
// ドメイン層。toasty の derive は付けない
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
pub struct UserId(Uuid);

// インフラストラクチャー層。Record の列は primitive のまま
pub(crate) struct UserRecord {
    #[key]
    pub id: Uuid,
}

// 境界で変換する
impl From<UserRecord> for User {
    fn from(record: UserRecord) -> Self {
        User {
            id: UserId::from(record.id),
            // ...
        }
    }
}
```

- **振る舞いは CRUD を基本とする。** 作成・取得・更新・削除を用意し、取得の条件はユースケースから決める(例: `get_user_by_sub`)。
- **スキーマの真実は Record。** `#[derive(toasty::Model)]` の Record と `#[table]` はインフラストラクチャー層にだけ置く。DDL は Record から生成し、手書きしない。
- **トレイトはドメインの言葉。** リポジトリーのメソッド名・引数・戻り値はドメインモデルで語る。エラーは `RepositoryError` に集約する。
- **変換は `From`。** Record とドメインモデルの対応は `impl From<Record> for Model` に書き、実装の各メソッドから再利用する。

## 参照実装

迷ったら既存コードを開く。このパターンの正がここにある。

- `apps/api/src/domain/model/email.rs` — 値オブジェクト(検証、`Display`、テスト)
- `apps/api/src/domain/model/user_id.rs` — 値オブジェクト(UUID の newtype、toasty 非依存)
- `apps/api/src/domain/model/user.rs` — 集約モデルと `new`
- `apps/api/src/domain/repository/user_repository.rs` — リポジトリーのトレイト
- `apps/api/src/domain/repository/errors.rs` — `RepositoryError`
- `apps/api/src/infrastructure/repository/user_repository.rs` — Record、`From`、実装、テスト
- `apps/api/src/infrastructure/repository/mod.rs` — `models()` への登録
- `apps/api/src/test_support/` — テスト用の DB セットアップ

## ツール

- マイグレーション: `apps/api` で `cargo run --bin toasty -- migration generate --name <snake_caseの名前>` → 生成された SQL を確認して `migration apply`
- テスト: `cargo test`(apps/api)

## よくある間違い

| 間違い | 正しいやり方 |
| --- | --- |
| DDL を手書きする | Record を編集して `migration generate` |
| Record を `models()` に登録し忘れる | 生成に現れない・接続時に落ちる。`infrastructure/repository/mod.rs` に登録する |
| ドメインモデルに `toasty::Model` や `toasty::Embed` を付ける | toasty の derive は infra の Record だけ。ドメインの newtype は toasty に依存せず、Record の列は primitive にして `From` で変換する |
| 取得系だけ作って更新・削除を省く | create / read / update / delete を揃える |
| エラーを `unwrap` する | `RepositoryError::DatabaseError` / `NotFound` に写像する |
