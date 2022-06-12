CREATE TABLE teams (
  id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  key TEXT NOT NULL,
  code TEXT NOT NULL,
  eliminated BOOLEAN NOT NULL DEFAULT 'f',
  champion BOOLEAN NOT NULL DEFAULT 'f'
);
