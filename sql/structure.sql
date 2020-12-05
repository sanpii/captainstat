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

create table if not exists statement (
    statement_id integer primary key,
    speaker_id integer references speaker,
    text text not null,
    time integer not null
);

create table if not exists source (
    url text primary key,
    language text,
    site_name text,
    title text
);

create table if not exists "user" (
    id integer primary key,
    achievements smallint[],
    mini_picture_url text not null,
    name text,
    picture_url text not null,
    registered_at timestamptz not null,
    reputation integer not null,
    speaker_id integer references speaker,
    username text not null
);

create table if not exists comment (
    comment_id integer primary key,
    approve bool,
    inserted_at timestamptz not null,
    is_reported bool not null,
    reply_to_id integer references comment deferrable,
    score integer,
    source_url text references source,
    statement_id integer not null,
    text text,
    user_id integer references "user"
);

commit;
