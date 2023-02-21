## features
- `test_api`テスト用のドメインを利用

## テスト
サーバーを起動
```
cd test_server
cargo run
```

テスト(別プロセス)
```
wasm-pack test --chrome --headless
```