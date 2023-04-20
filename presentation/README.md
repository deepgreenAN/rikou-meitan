## Features
- `develop` 開発中であることを明示してビルド
- `fake` フェイクデータを利用する
- `test_api` テストサーバー(インメモリ・CORSを許容)を利用する。
- なし(本番ビルド)

## Examples
`examples/hello_world.rs`を実行する例．その他の設定は`Dioxus.toml`にある
```
dioxus serve --example hello_world
```

## Dev
設定は`Trunk.toml`にある．

### Fake
APIはサーバーは利用せずフェイクデータを用いる．

```
trunk serve
```

### テストAPI
```
trunk serve trunk_index_develop.html
```

## Release
設定は`Trunk.toml`にある．

### SPA
```
trunk build trunk_index_spa.html --release
```
### SSR
```
trunk build trunk_index_ssr.html --release --dist dist_ssr
```

