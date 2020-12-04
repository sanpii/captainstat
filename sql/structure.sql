begin;

create table if not exists speaker (
    speaker_id integer primary key,
    full_name text not null,
    slug text
);

create table if not exists video (
    video_id integer primary key,
    hash_id text not null,
    inserted_at timestamptz not null,
    is_partner bool not null,
    speaker_ids integer[],
    thumbnail text not null,
    title text not null,
    youtube_id text not null
);

commit;
