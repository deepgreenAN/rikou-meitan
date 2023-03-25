# application/serverside
## テスト
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
cargo run --bin test_server --features inmemory
```