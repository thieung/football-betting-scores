table! {
    bets (id) {
        id -> Uuid,
        game_id -> Uuid,
        user_id -> Uuid,
        score_ids -> Jsonb,
        amount_bet -> Int4,
        amount_win -> Int4,
        locked -> Bool,
        lasted_changed_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    games (id) {
        id -> Uuid,
        round_id -> Uuid,
        group_id -> Uuid,
        team1_id -> Nullable<Uuid>,
        team2_id -> Nullable<Uuid>,
        unkown_team1_title -> Nullable<Text>,
        unkown_team2_title -> Nullable<Text>,
        pos -> Int4,
        play_at -> Nullable<Timestamp>,
        score1 -> Nullable<Int4>,
        score2 -> Nullable<Int4>,
        score_id -> Nullable<Uuid>,
        winner_id -> Nullable<Uuid>,
        locked -> Bool,
        reason -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    groups (id) {
        id -> Uuid,
        name -> Nullable<Text>,
        pos -> Int4,
    }
}

table! {
    investments (id) {
        id -> Uuid,
        game_id -> Uuid,
        total -> Int4,
        remaining -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    predict_champions (id) {
        id -> Uuid,
        user_id -> Uuid,
        team_id -> Uuid,
        amount -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    rounds (id) {
        id -> Uuid,
        name -> Nullable<Text>,
        pos -> Int4,
        start_at -> Timestamp,
        end_at -> Timestamp,
    }
}

table! {
    scores (id) {
        id -> Uuid,
        name -> Text,
        score1 -> Text,
        score2 -> Text,
    }
}

table! {
    teams (id) {
        id -> Uuid,
        key -> Text,
        code -> Text,
        eliminated -> Bool,
        champion -> Bool,
    }
}

table! {
    user_preferences (id) {
        id -> Uuid,
        user_id -> Uuid,
        listen_music -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Uuid,
        email -> Text,
        username -> Text,
        password -> Text,
        full_name -> Nullable<Text>,
        nickname -> Nullable<Text>,
        active -> Bool,
        allied -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

joinable!(bets -> games (game_id));
joinable!(bets -> users (user_id));
joinable!(games -> groups (group_id));
joinable!(games -> rounds (round_id));
joinable!(games -> scores (score_id));
joinable!(investments -> games (game_id));
joinable!(predict_champions -> teams (team_id));
joinable!(predict_champions -> users (user_id));
joinable!(user_preferences -> users (user_id));

allow_tables_to_appear_in_same_query!(
    bets,
    games,
    groups,
    investments,
    predict_champions,
    rounds,
    scores,
    teams,
    user_preferences,
    users,
);
