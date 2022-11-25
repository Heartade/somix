use yew::prelude::*;
use yewdux::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Default, PartialEq, Clone, Serialize, Deserialize, Store)]
#[store(storage = "local", storage_tab_sync)]
pub struct UserStore {
    pub user_id: String,
    pub device_id: String,
    pub access_token: String,
    pub home_server: String
}
