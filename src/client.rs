use gloo_console::log;
use matrix_sdk::{
    config::SyncSettings,
    deserialized_responses::SyncResponse,
    ruma::{
        events::{
            room::message::{MessageType, OriginalSyncRoomMessageEvent, SyncRoomMessageEvent},
            AnySyncMessageLikeEvent, AnySyncTimelineEvent, SyncMessageLikeEvent,
        },
        user_id, RoomId,
    },
    Client, LoopCtrl,
};

pub struct MatrixSocialClient(pub Client);

impl MatrixSocialClient {
    pub async fn on_room_message(&self, room_id: &RoomId, event: &OriginalSyncRoomMessageEvent) {
        let MessageType::Text(text_content) = &event.content.msgtype else { return };

        log!(&format!("received event {:?}", &text_content.body).to_string());
    }

    pub async fn on_sync_response(&self, response: SyncResponse) -> LoopCtrl {
        log!("synced");

        for (room_id, room) in response.rooms.join {
            for event in room.timeline.events {
                if let Ok(AnySyncTimelineEvent::MessageLike(
                    AnySyncMessageLikeEvent::RoomMessage(SyncMessageLikeEvent::Original(ev)),
                )) = event.event.deserialize()
                {
                    self.on_room_message(&room_id, &ev).await
                }
            }
        }

        LoopCtrl::Continue
    }
}
