# application/server
## テスト
ハンドラを除く
```
cargo test
```

## テストサーバーを立てる
データベース
```
cargo run --bin test_server
```
インメモリ
```
cargo run --bin test_server --feature inmemory
```