use gloo_storage::{errors::StorageError, LocalStorage, Storage};

use std::collections::HashSet;

// -------------------------------------------------------------------------------------------------
// liked_ids関連

pub fn get_liked_ids() -> Result<HashSet<String>, StorageError> {
    let res = LocalStorage::get::<HashSet<String>>("liked_ids");
    match res {
        Ok(liked_ids_set) => Ok(liked_ids_set),
        Err(e) => {
            match e {
                // ストレージに対応する要素が存在しない場合
                StorageError::KeyNotFound(_) => {
                    // 新しいセットを追加
                    let new_set = HashSet::<String>::new();
                    LocalStorage::set("liked_ids", new_set.clone())?;
                    Ok(new_set)
                }
                // その他のエラー
                e => Err(e),
            }
        }
    }
}

pub fn push_liked_id(id: String) -> Result<(), StorageError> {
    let res = LocalStorage::get::<HashSet<String>>("liked_ids");
    match res {
        Ok(mut liked_ids_set) => {
            liked_ids_set.insert(id);
            LocalStorage::set("liked_ids", liked_ids_set)?;
            Ok(())
        }
        Err(e) => {
            match e {
                // ストレージに対応する要素が存在しない場合
                StorageError::KeyNotFound(_) => {
                    // 新しいセットを追加
                    let mut new_set = HashSet::<String>::new();
                    new_set.insert(id);
                    LocalStorage::set("liked_ids", new_set)?;
                    Ok(())
                }
                // その他のエラー
                e => Err(e),
            }
        }
    }
}
