# presentation

## Features

- `develop` 開発中であることを明示してビルド
- `fake` フェイクデータを利用する
- `test_api` テストサーバー(インメモリ・CORSを許容)を利用する。
- なし(本番ビルド)

## Examples

`examples/hello_world.rs`を実行する例．その他の設定は`Dioxus.toml`にある

```shell
dioxus serve --example hello_world
```

## Dev

設定は`Trunk.toml`にある．

### Fake

APIはサーバーは利用せずフェイクデータを用いる．

```shell
trunk serve
```

### テストAPI

```shell
trunk serve trunk_index_test_api.html
```

## Release

設定は`Trunk.toml`にある．

### SPA

```shell
trunk build trunk_index_spa.html --release --dist ../dist_spa
```

### SSR

```shell
trunk build trunk_index_ssr.html --release --dist ../dist_ssr
```
