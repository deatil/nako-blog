use std::collections::HashMap;

use crate::nako::{
    redis,
    global::AppState,
};
use crate::app::model::{
    setting,
};

// 全部设置
pub async fn settings(state: &mut AppState) -> HashMap<String, String> {
    let db = &state.db;
    let r = &mut state.redis;

    let setting_key = "nako:setting_key";

    let res = redis::get::<HashMap<String, String>>(r, setting_key).await;
    if res.is_ok() {
        if let Some(v) = res.unwrap() {
            return v;
        };
    }

    let settings = setting::SettingModel::find_all(db).await.unwrap_or_default();

    let mut data = HashMap::new();
    if settings.len() > 0 {
        for setting in settings {
            data.insert(setting.key, setting.value);
        }
    }

    let _ = redis::set::<HashMap<String, String>>(r, setting_key, data.clone(), 60*60*24*2);

    data
}

// 清空设置缓存
pub async fn clear(state: &mut AppState) {
    let r = &mut state.redis;

    let setting_key = "nako:setting_key".to_string();

    let _ = redis::delete(r, vec![setting_key]).await;
}
