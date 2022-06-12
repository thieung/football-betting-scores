CREATE TABLE games (
  id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  round_id UUID NOT NULL REFERENCES rounds (id),
  group_id UUID NOT NULL REFERENCES groups (id),
  team1_id UUID REFERENCES teams (id),
  team2_id UUID REFERENCES teams (id),
  unkown_team1_title TEXT,
  unkown_team2_title TEXT,
  pos INTEGER NOT NULL,
  play_at TIMESTAMP,
  score1 INTEGER,
  score2 INTEGER,
  score_id UUID REFERENCES scores (id),
  winner_id UUID REFERENCES teams (id),
  locked BOOLEAN NOT NULL DEFAULT 'f',
  reason TEXT,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
  updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE INDEX games_round_id_idx ON games (round_id);
CREATE INDEX games_group_idx ON games (group_id);
CREATE INDEX games_team1_id_idx ON games (team1_id);
CREATE INDEX games_team2_id_idx ON games (team2_id);
CREATE INDEX games_score_id_idx ON games (score_id);
CREATE INDEX games_winner_id_idx ON games (winner_id);
