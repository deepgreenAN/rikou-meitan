## features
- `test_api` テスト用のドメインを利用
- `fake` APIを利用せずフェイクデータを取得
- `integration_test` 結合テストをコンパイルする。単体テストは含めない。

## 単体テスト

### 通常
```
wasm-pack test --chrome --headless
```
### fake
```
wasm-pack test --chrome --headless -- --features fake
```

## 結合テスト
```
wasm-pack test --chrome --headless -- --features integration_test --features test_api
```