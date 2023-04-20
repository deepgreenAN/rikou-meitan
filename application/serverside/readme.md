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

## サーバー

### APIサーバーのみ
データベース
```
cargo run --bin api_test_server
```
インメモリ
```
cargo run --bin api_test_server --features inmemory
```

### SPA
データベース
```
cargo run --bin spa
```
インメモリ
```
cargo run --bin spa --features inmemory
```

### SSR
データベース
```
cargo run --bin ssr
```
インメモリ
```
cargo run --bin ssr --features inmemory
```