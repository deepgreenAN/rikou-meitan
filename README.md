# 莉光迷站

おりコウのファンページ．開発中．

## フレームワーク

- フロントエンド: [dioxus](https://github.com/DioxusLabs/dioxus)
- webサーバー: [axum](https://github.com/tokio-rs/axum)
- データベース: [sqlx](https://github.com/launchbadge/sqlx)

## ロードマップ

- ✅ 同期的・個数の決まったプレーヤー
- ✅ 単一のクリップ
- ✅ SSR
- ✅ エピソードのHtmlの描画
- ✅ XSS対策
- ✅ hydration
- ⬜ 一連のクリップ
- ⬜ 投稿における認証
- ⬜ サーバー側・フロントエンドのエラーの表示
- ⬜ トレーシング

## バグ

- ✅ SSRの際に余計な文字列が生成される
- ⬜ モーダルの解除イベントにmouseupが使えない

## Run

### データベースの起動・マイグレーション

```shell
nu postgre_start.nu
```

```shell
sqlx database create
sqlx migrate run
```

### フロントエンドのコンパイル

```shell
nu build_dist.nu
```

### ローカルでサーバーをビルド

```shell
source env.nu
cd application/serverside
cargo run --example ssr
```

### ローカルでサーバーをビルド(shuttle)

```shell
cargo shuttle run
```

### デプロイ

```shell
nu deploy.nu
```
