## データベースの開始
```
nu ../postgre_start.nu
source ../env.nu
```
## データベースの作成
```
sqlx database create
```

## マイグレーション
マイグレーションを実行
```
cd ..
sqlx migrate run
cd infrastructure
```

## データベースの削除
```
sqlx database drop
```

## テスト
```
cargo test
cargo test -- --ignored
```

## データベースの終了
```
nu ../postgre_stop.nu
```