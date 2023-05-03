# application/serverside

## テスト

```shell
cargo test
```

## データベース

```shell
cd ../..
nu postgre_start.nu
```

```shell
source ../../env.nu
```

## サーバー

### APIサーバーのみ

データベース

```shell
cargo run --example api_test_server
```

インメモリ

```shell
cargo run --example api_test_server --features inmemory
```

### SPA

データベース

```shell
cargo run --example spa
```

インメモリ

```shell
cargo run --example spa --features inmemory
```

### SSR

データベース

```shell
cargo run --example ssr --features ssr
```

インメモリ

```shell
cargo run --example ssr --features inmemory
```
