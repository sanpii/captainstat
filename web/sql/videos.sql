with v as (
    select video.title, video.thumbnail as picture, video.posted_at,
            'https://captainfact.io/videos/' || video.hash_id as url,
            count(1) filter (where comment.approve) as nb_approves,
            count(1) filter (where not comment.approve) as nb_refutes,
            count(1) filter (where comment.approve is null) as nb_comments,
            count(1)::numeric as total
        from video
        left join statement using(video_id)
        left join comment using(statement_id)
        group by video.title, video.thumbnail, video.hash_id, video.posted_at
)
select title, picture, url, nb_approves, nb_refutes, nb_comments,
        round(nb_approves::numeric / total * 100.0)::float4 as percent_approves,
        round(nb_refutes::numeric / total * 100.0)::float4 as percent_refutes,
        round(nb_comments::numeric / total * 100.0)::float4 as percent_comments
    from v
    order by posted_at desc
    offset $* fetch first $* rows only
