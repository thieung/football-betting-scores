CREATE TABLE scores (
  id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  name TEXT NOT NULL,
  score1 TEXT NOT NULL,
  score2 TEXT NOT NULL
);
