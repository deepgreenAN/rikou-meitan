# application/serverside
## テスト
```
cargo test
```

## データベース
```
cd ../..
nu postgre_start.nu
```

```
source ../../env.nu
```

## サーバー

### APIサーバーのみ
データベース
```
cargo run --example api_test_server
```
インメモリ
```
cargo run --example api_test_server --features inmemory
```

### SPA
データベース
```
cargo run --example spa
```
インメモリ
```
cargo run --example spa --features inmemory
```

### SSR
データベース
```
cargo run --example ssr --features ssr
```
インメモリ
```
cargo run --example ssr --features inmemory, ssr
```