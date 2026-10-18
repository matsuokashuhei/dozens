---
name: toasty-domain-repository
description: Use when adding or extending a persistent domain entity in a Rust application that uses Toasty and layered domain and infrastructure repositories.
---

# Toasty ドメインモデルとリポジトリー

## 目的

既存の Rust / Toasty アプリケーションの設計に合わせて、永続化対象のドメインモデルとリポジトリーを一貫して追加する。データベースの構造をドメイン層から分離し、ドメインで意味のある値と操作を表現する。

## ゴール

対象のエンティティについて、次の成果物を揃える。

- `src/domain/model` にドメインモデルと必要な値型がある。
- `src/domain/repository` にドメイン層のリポジトリー契約がある。
- `src/infrastructure/repository` に Toasty を使うリポジトリー実装がある。
- `toasty/migrations` に対応するスキーマ変更のマイグレーションがある。
- リポジトリー契約が対象モデルの CRUD（作成・取得・更新・削除）を表現する。

## 前提

- ディレクトリー構成、命名、エラー型、非同期処理、依存関係の登録方法は、対象リポジトリーの既存パターンを基準にする。
- テーブル設計は対象のドメイン要件と既存データモデルに基づく。既存の関連や一意性、インデックス、生成列の扱いを確認し、根拠のない制約を加えない。
- 特定の意味を持つ値は、プリミティブ型のまま広く受け渡さず、ドメイン上の型として表現する。例として ID、Eメール、位置情報などがある。型の検証や変換は、その値に適用されるルールに応じて定める。
- Toasty のモデルやデータベース固有の型はインフラストラクチャー層に置き、ドメインモデルが永続化実装に依存しないようにする。

## 設計指針

- 値型は、ドメインでの意味と不変条件を明確にする。単なる別名で十分か、生成時の検証や安全な変換が必要かは既存のドメイン規則に従う。
- インフラストラクチャー層の永続化レコードからドメインモデルへの変換を `From` トレイトで実装する。データベース表現の違いはこの境界で吸収する。
- リポジトリーの契約はドメインモデルとドメイン上の操作を扱い、Toasty の型やレコード型を公開しない。CRUD の各操作は、対象の識別方法や戻り値、既存エラー規約に合わせて表現する。
- Toasty のモデル定義、関連、テーブル名、制約は実際のスキーマと整合させる。マイグレーションはモデル定義と同じ変更を反映する。
- 実装に必要なモジュール宣言、モデル登録、依存注入などの接続箇所も既存構成に沿って更新し、成果物がアプリケーションから利用可能な状態にする。
- 既存コードを再利用しつつ、要求されていないレイヤー変更や一括リファクタリングは避ける。

## 参照例

このリポジトリーでは `apps/api/src/domain/model` の `email.rs` と `user.rs`、`apps/api/src/domain/repository` の各リポジトリー、`apps/api/src/infrastructure/repository` の各実装、および `apps/api/toasty/migrations` を既存パターンの例として参照する。適用先が異なる場合は、そのリポジトリー内の同等の実装を優先する。

値型の形の例:

```rust
use toasty::Model;
use uuid::Uuid;

#[derive(Debug, toasty::Embed)]
struct Email(String);

#[derive(Debug, toasty::Embed)]
struct UserId(Uuid);
```

この例は形式を示すものであり、値型を Toasty の埋め込み型にするか、モデル側でどう永続化するかは既存実装と Toasty の制約に合わせる。
