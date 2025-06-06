CREATE TABLE users (
    id              INTEGER PRIMARY KEY NOT NULL GENERATED ALWAYS AS IDENTITY,
    username        VARCHAR(255) NOT NULL,
    password_hash   BYTEA NOT NULL,
    salt            BYTEA NOT NULL,

    UNIQUE (username)
);

CREATE INDEX ON users(username);

CREATE TABLE tokens (
    id              INTEGER PRIMARY KEY NOT NULL GENERATED ALWAYS AS IDENTITY,
    user_id         INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    token           CHAR(344) NOT NULL,
    expires         TIMESTAMP WITH TIME ZONE NOT NULL
);

CREATE INDEX on tokens(token);

CREATE TABLE contests (
    id              INTEGER PRIMARY KEY NOT NULL GENERATED ALWAYS AS IDENTITY,
    phiquadro_id    INTEGER NOT NULL,
    phiquadro_sess  INTEGER NOT NULL,
    contest_name    VARCHAR(255) NOT NULL,
    duration        INTEGER NOT NULL,
    start_time      TIMESTAMP WITH TIME ZONE NOT NULL,
    drift           INTEGER NOT NULL,
    drift_time      INTEGER NOT NULL,
    jolly_time      INTEGER NOT NULL,
    teams_no        INTEGER NOT NULL,
    questions_no    INTEGER NOT NULL,
    active          BOOLEAN NOT NULL,
    question_bonus  INTEGER[10] NOT NULL CHECK (array_position(question_bonus, NULL) IS NULL),
    contest_bonus   INTEGER[10] NOT NULL CHECK (array_position(contest_bonus, NULL) IS NULL),
    owner_id        INTEGER NOT NULL REFERENCES users(id),

    CONSTRAINT positive_duration CHECK (duration >= 0),
    CONSTRAINT positive_drift CHECK (drift >= 0),
    CONSTRAINT positive_drift_time CHECK (drift_time >= 0),
    CONSTRAINT positive_jolly_time CHECK (jolly_time >= 0),
    CONSTRAINT positive_teams CHECK (teams_no >= 0),
    CONSTRAINT positive_questions CHECK (questions_no >= 0),
    CONSTRAINT reasonable_drift_time CHECK (drift_time <= duration),
    CONSTRAINT reasonable_jolly_time CHECK (jolly_time <= duration)
);

CREATE TABLE questions(
    id              INTEGER PRIMARY KEY NOT NULL GENERATED ALWAYS AS IDENTITY,
    answer          INTEGER NOT NULL,
    position        INTEGER NOT NULL,
    contest_id      INTEGER NOT NULL REFERENCES contests(id),

    UNIQUE (contest_id, position),
    CONSTRAINT positive_id CHECK (id >= 0),
    CONSTRAINT positive_position CHECK (position >= 0)
);

CREATE INDEX ON questions (contest_id);

CREATE TABLE teams(
    id              INTEGER PRIMARY KEY NOT NULL GENERATED ALWAYS AS IDENTITY,
    team_name       VARCHAR(255) NOT NULL,
    is_fake         BOOLEAN NOT NULL,
    position        INTEGER NOT NULL,
    contest_id      INTEGER NOT NULL REFERENCES contests(id),

    UNIQUE (contest_id, position),
    CONSTRAINT positive_id CHECK (id >= 0),
    CONSTRAINT positive_position CHECK (position >= 0)
);

CREATE INDEX ON teams (contest_id);

CREATE TABLE submissions(
    id              INTEGER PRIMARY KEY NOT NULL GENERATED ALWAYS AS IDENTITY,
    answer          INTEGER NOT NULL,
    sub_time        TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    team_id         INTEGER NOT NULL REFERENCES teams(id),
    question_id     INTEGER NOT NULL REFERENCES questions(id)
);

CREATE INDEX ON submissions (team_id);

CREATE TABLE jollies (
    id              INTEGER PRIMARY KEY NOT NULL GENERATED ALWAYS AS IDENTITY,
    sub_time        TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    team_id         INTEGER NOT NULL REFERENCES teams(id),
    question_id     INTEGER NOT NULL REFERENCES questions(id),

    UNIQUE (team_id)
);

CREATE INDEX ON jollies (team_id);
