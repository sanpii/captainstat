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
    video_id integer references video,
    text text not null,
    time integer not null
);

create index if not exists statement_speaker_id on statement(speaker_id);
create index if not exists statement_video_id on statement(video_id, statement_id);

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

create index if not exists user_speaker_id on "user"(speaker_id);

create table if not exists comment (
    comment_id integer primary key,
    approve bool,
    inserted_at timestamptz not null,
    is_reported bool not null,
    reply_to_id integer references comment,
    score integer,
    source_url text references source,
    statement_id integer not null references statement,
    text text,
    user_id integer references "user"
);

create index if not exists comment_reply_to_id on comment((1)) where reply_to_id is null;
create index if not exists comment_source_url on comment(source_url);
create index if not exists comment_statement_id on comment(statement_id);
create index if not exists comment_user_id on comment(user_id);

create schema if not exists view;

create or replace view view.comment as
    select
        case when comment.score <= 0 or (comment.score is null and comment.comment_id is not null)
        then
            1.0
        else
            comment.score + 1.0
        end as score,
        comment.approve,
        comment.comment_id,
        comment.statement_id
    from comment
    where reply_to_id is null;

create or replace view view.video as
    with v as (
        select video.title, video.thumbnail as picture, video.posted_at,
                'https://captainfact.io/videos/' || video.hash_id as url,
                count(comment.comment_id) filter (where comment.approve) as nb_approves,
                count(comment.comment_id) filter (where not comment.approve) as nb_refutes,
                count(comment.comment_id) filter (where comment.approve is null) as nb_comments,
                sum(comment.score) filter (where comment.approve) as score_approves,
                sum(comment.score) filter (where not comment.approve) as score_refutes,
                sum(comment.score) filter (where comment.approve is null) as score_comments,
                sum(comment.score) as total
            from video
            left join statement using(video_id)
            left join view.comment comment using(statement_id)
            group by video.title, video.thumbnail, video.hash_id, video.posted_at
    )
    select title, picture, url, nb_approves, nb_refutes, nb_comments, total,
            round(score_approves / total * 100.0, 2)::float4 as percent_approves,
            round(score_refutes / total * 100.0, 2)::float4 as percent_refutes,
            round(score_comments / total * 100.0, 2)::float4 as percent_comments
        from v
        order by posted_at desc;

create or replace view view.speaker as
    with s as (
        select speaker.full_name as title, speaker.picture,
                'https://captainfact.io/s/' || speaker.speaker_id as url,
                count(comment.comment_id) filter (where comment.approve) as nb_approves,
                count(comment.comment_id) filter (where not comment.approve) as nb_refutes,
                count(comment.comment_id) filter (where comment.approve is null) as nb_comments,
                sum(comment.score) filter (where comment.approve) as score_approves,
                sum(comment.score) filter (where not comment.approve) as score_refutes,
                sum(comment.score) filter (where comment.approve is null) as score_comments,
                sum(comment.score) as total
            from speaker
            left join statement using(speaker_id)
            left join video using(video_id)
            left join view.comment comment using(statement_id)
            group by speaker.full_name, speaker.speaker_id, speaker.picture
    )
    select title, url, picture, nb_approves, nb_refutes, nb_comments,
            round(score_approves / total * 100.0, 2)::float4 as percent_approves,
            round(score_refutes / total * 100.0, 2)::float4 as percent_refutes,
            round(score_comments / total * 100.0, 2)::float4 as percent_comments
        from s
        order by title;

commit;
