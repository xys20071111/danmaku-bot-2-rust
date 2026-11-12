pub mod gift {
    include!(concat!(env!("OUT_DIR"), "/dm_proto.gift.rs"));
}

pub mod user_info {
    include!(concat!(env!("OUT_DIR"), "/dm_proto.user_info.rs"));
}

pub mod interact_word {
    include!(concat!(env!("OUT_DIR"), "/dm_proto.interact_word.rs"));
}
