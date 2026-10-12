# Dozens iOS

[ikzo-ios](https://github.com/matsuokashuhei/ikzo-ios)（UIKit + Storyboard + CocoaPods、2019 年）を作り直した iOS アプリ。

## 必要なもの

- Xcode 26 以上（Swift 6.2）
- iOS 26 以上
- [XcodeGen](https://github.com/yonaskolb/XcodeGen)（`project.yml` を変えたときだけ）

## 起動

```sh
cd apps/ios
open Dozens.xcodeproj
```

接続先は `project.yml` の `API_BASE_URL` で決まる（デフォルトは `http://localhost:3000`、`apps/api` の `cargo run --bin api`）。
API を立てずに UI を触るときは、スキーム `Dozens` の Run > Environment Variables で `DOZENS_BACKEND=stub` を有効にする。スタブの OTP は `123456`。

`project.yml` を変えたら `xcodegen generate` で `Dozens.xcodeproj` を作り直してコミットする（CI が差分を検査する）。

## テスト

```sh
xcodebuild test -project Dozens.xcodeproj -scheme Dozens \
  -destination 'platform=iOS Simulator,name=iPhone 17'
```

テストは Swift Testing で書く。

## 構成

アプリターゲット（`App/`）は `AppRootView` を表示するだけにして、コードは全部ローカル SPM パッケージ `Packages/DozensKit` に置く。レイヤーは `apps/api` と揃えている。

| モジュール | 役割 | 依存 |
| --- | --- | --- |
| `Domain` | エンティティ（`User`, `Email`, `OneTimeCode`, `AuthTokens`）と Repository プロトコル | なし |
| `Application` | ユースケース（`PasswordlessAuthenticator`）とアプリ状態（`SessionStore`） | Domain |
| `Infrastructure` | `APIClient`（URLSession + async/await）、`APIAuthRepository`、`KeychainTokenStore`、スタブ実装 | Domain |
| `DesignSystem` | カラー、Futura のタイポグラフィ、ロゴ | なし |
| `AuthFeature` | メール OTP のサインイン/サインアップ画面 | Domain, Application, DesignSystem |
| `AppFeature` | ルート画面、タブ、依存関係の組み立て | 全部 |

- Swift 6 language mode（strict concurrency）。UI モジュールは `defaultIsolation(MainActor.self)`
- 状態は `@Observable`、画面遷移は `NavigationStack(path:)`、エラーは typed throws（`throws(AuthError)`）で表す
- サードパーティ依存は使わない
- 文字列は String Catalog（`Localizable.xcstrings`）で管理する。英語がキー、日本語が翻訳

### 認証フロー

1. メールアドレスを入力すると `POST /sign_in` を呼ぶ。`user not found` / `user not confirmed` が返ったら `POST /sign_up` に切り替える
2. 6 桁のコードを入力すると `POST /confirm_sign_in` または `POST /confirm_sign_up` を呼び、トークンを受け取る
3. `GET /user` でユーザーを取得し、トークンを Keychain に保存する
4. サインアウトは `DELETE /sign_out` を呼んでから Keychain を消す

## 移行状況

| 旧アプリの機能 | 状態 |
| --- | --- |
| サインイン/サインアップ（Firebase, Facebook, Google, Twitter） | メール OTP に置き換え済み |
| タイムライン、動画再生 | 未着手 |
| ビート選択、録音、フィルタ、投稿 | 未着手 |
| 検索、通知、プロフィール、フォロー、設定 | 未着手 |
