# application/server
## テスト
ハンドラを除く
```
cargo test
```
ハンドラを含める
```
cargo test --features mock
```

## テストサーバーを立てる
```
cargo run --bin test_server --features mock
```