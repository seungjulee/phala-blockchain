use chain::BlockNumber;
use phala_mq::traits::MessagePrepareChannel;
use phala_mq::SignedMessageChannel;

use crate::side_task::SideTaskManager;

pub fn process_block(
    block_number: BlockNumber,
    egress: &SignedMessageChannel,
    side_task_man: &mut SideTaskManager,
) {
    if block_number % 2 == 1 {
        let egress = egress.clone();
        let default_messages = [egress.prepare_message_to(&"Timeout", "a/mq/topic")];
        side_task_man.add_async_task_finish_at(
            block_number + 3,
            default_messages,
            async move {
                log::info!("Side task start to get ip");
                let mut response = surf::get("https://ip.kvin.wang").send().await.unwrap();
                let ip = response.body_string().await.unwrap();
                log::info!("Side task got ip: {}", ip);
                Ok([egress.prepare_message_to(&ip, "a/mq/topic")])
            },
        );
    }
}
