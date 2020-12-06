with s as (
    select speaker.full_name as title, speaker.picture,
            'https://captainfact.io/s/' || speaker.speaker_id as url,
            count(distinct statement.statement_id) as count_statement,
            count(1) filter (where comment.approve) as nb_approves,
            count(1) filter (where not comment.approve) as nb_refutes,
            count(1) filter (where comment.approve is null) as nb_comments,
            count(1)::numeric as total
    from speaker
    left join statement using(speaker_id)
    left join video using(video_id)
    left join comment using(statement_id)
    group by speaker.full_name, speaker.speaker_id, speaker.picture
)
select title, url, picture, nb_approves, nb_refutes, nb_comments,
        round(nb_approves::numeric / total * 100.0)::float4 as percent_approves,
        round(nb_refutes::numeric / total * 100.0)::float4 as percent_refutes,
        round(nb_comments::numeric / total * 100.0)::float4 as percent_comments
    from s
    order by title
    offset $* fetch first $* rows only
