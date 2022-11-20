CREATE TABLE IF NOT EXISTS movie_clips (
    title TEXT NOT NULL,
    "url" TEXT NOT NULL,
    "start" INT4 NOT NULL,
    "end" INT4 NOT NULL,
    id uuid PRIMARY KEY NOT NULL,
    "like" INT4 NOT NULL,
    dislike INT4 NOT NULL
);
CREATE TABLE IF NOT EXISTS episodes (
    "date" DATE NOT NULL,
    content TEXT NOT NULL,
    id uuid PRIMARY KEY NOT NULL
);