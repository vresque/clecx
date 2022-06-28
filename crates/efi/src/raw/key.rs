use crate::status::EfiResult;

use super::text::{KeyCode, TextKey};
use crate::table;

pub fn efi_key(shall_wait: bool) -> EfiResult<TextKey> {
    if shall_wait {
        let mut tmp = 0;
        (table().boot.wait_for_event)(1, &table().con_in.wait_for_key, &mut tmp).result()?;
    }

    let mut key: TextKey = TextKey::default();
    (table().con_in.read_key_stroke)(table().con_in, &mut key).result()?;
    Ok(key)
}

pub fn key(shall_wait: bool) -> EfiResult<KeyCode> {
    Ok(KeyCode::from(efi_key(shall_wait)?))
}
