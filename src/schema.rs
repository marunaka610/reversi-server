table! {
    game_infos (game_id) {
        game_id -> Integer,
        state -> Integer,
        start_time -> Timestamp,
        end_time -> Nullable<Timestamp>,
    }
}

table! {
    progresses (game_id, piecies) {
        game_id -> Integer,
        progress -> Integer,
        piecies -> Binary,
        time -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(game_infos, progresses,);
