# excludeされたディレクトリのtargetを削除．
if ("presentation/target" | path exists) { rm "presentation/target" --recursive --interactive-once }
if ("application/frontend/target" | path exists) { rm "application/frontend/target" --recursive --interactive-once }
if ("application/integration_test/target" | path exists) { rm "application/integration_test/target" --recursive --interactive-once }

# デプロイ
cargo shuttle deploy --no-test