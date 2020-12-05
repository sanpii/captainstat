begin;

create table if not exists speaker (
    speaker_id integer primary key,
    full_name text not null,
    slug text,
    country text,
    picture text,
    title text,
    wikidata_item_id text
);

create table if not exists video (
    video_id integer primary key,
    hash_id text not null,
    is_partner bool not null,
    speaker_ids integer[],
    thumbnail text not null,
    title text not null,
    youtube_id text,
    posted_at timestamptz not null,
    language text,
    provider text not null,
    provider_id text,
    url text not null,
    youtube_offset integer not null
);

commit;
