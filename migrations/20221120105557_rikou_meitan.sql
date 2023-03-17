-- movie_clipsテーブルについて
CREATE TABLE IF NOT EXISTS movie_clips (
    title TEXT NOT NULL,
    "url" TEXT NOT NULL,
    "start" INT4 NOT NULL,
    "end" INT4 NOT NULL,
    id uuid PRIMARY KEY NOT NULL,
    "like" INT4 NOT NULL,
    create_date DATE NOT NULL
);

-- episodesテーブルについて
CREATE TABLE IF NOT EXISTS episodes (
    "date" DATE NOT NULL,
    content TEXT NOT NULL,
    id uuid PRIMARY KEY NOT NULL
);

-- videoテーブルについて
CREATE TABLE IF NOT EXISTS videos (
    title TEXT NOT NULL,
    "url" TEXT NOT NULL,
    id uuid PRIMARY KEY NOT NULL,
    "date" DATE NOT NULL,
    author TEXT NOT NULL,
    "like" INT4 NOT NULL,
    video_type TEXT NOT NULL
);