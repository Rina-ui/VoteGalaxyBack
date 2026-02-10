-- Add migration script here
-- Table des admins
CREATE TABLE IF NOT EXISTS admins (
                                      id TEXT PRIMARY KEY,
                                      username TEXT NOT NULL UNIQUE,
                                      password_hash TEXT NOT NULL
);

-- Table des polls
CREATE TABLE IF NOT EXISTS polls (
                                     id TEXT PRIMARY KEY,
                                     question TEXT NOT NULL,
                                     created_by TEXT NOT NULL,
                                     created_at TEXT NOT NULL,
                                     closes_at TEXT,
                                     is_active BOOLEAN NOT NULL DEFAULT 1,
                                     FOREIGN KEY (created_by) REFERENCES admins(id)
);



-- Table des options de vote
CREATE TABLE IF NOT EXISTS poll_options (
                                            id TEXT PRIMARY KEY,
                                            poll_id TEXT NOT NULL,
                                            text TEXT NOT NULL,
                                            vote_count INTEGER NOT NULL DEFAULT 0,
                                            FOREIGN KEY (poll_id) REFERENCES polls(id) ON DELETE CASCADE
);

-- Table des votes
CREATE TABLE IF NOT EXISTS votes (
                                     id TEXT PRIMARY KEY,
                                     poll_id TEXT NOT NULL,
                                     option_id TEXT NOT NULL,
                                     voter_id TEXT,
                                     voted_at TEXT NOT NULL,
                                     FOREIGN KEY (poll_id) REFERENCES polls(id) ON DELETE CASCADE,
    FOREIGN KEY (option_id) REFERENCES poll_options(id) ON DELETE CASCADE
);

-- Index pour améliorer les performances
CREATE INDEX IF NOT EXISTS idx_polls_created_by ON polls(created_by);
CREATE INDEX IF NOT EXISTS idx_poll_options_poll_id ON poll_options(poll_id);
CREATE INDEX IF NOT EXISTS idx_votes_poll_id ON votes(poll_id);
CREATE INDEX IF NOT EXISTS idx_votes_option_id ON votes(option_id);