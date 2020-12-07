with
score as (
    select
        case when comment.score <= 0 or (comment.score is null and comment.comment_id is not null)
        then
            1.0
        else
            comment.score + 1.0
        end as score,
        comment.approve,
        comment.comment_id
    from comment
),
s as (
    select speaker.full_name as title, speaker.picture,
            'https://captainfact.io/s/' || speaker.speaker_id as url,
            count(comment.comment_id) filter (where comment.approve) as nb_approves,
            count(comment.comment_id) filter (where not comment.approve) as nb_refutes,
            count(comment.comment_id) filter (where comment.approve is null) as nb_comments,
            sum(score.score) filter (where comment.approve) as score_approves,
            sum(score.score) filter (where not comment.approve) as score_refutes,
            sum(score.score) filter (where comment.approve is null) as score_comments,
            sum(score.score) as total
        from speaker
        left join statement using(speaker_id)
        left join video using(video_id)
        left join comment using(statement_id)
        left join score using(comment_id)
        where comment.reply_to_id is null
        group by speaker.full_name, speaker.speaker_id, speaker.picture
)
select title, url, picture, nb_approves, nb_refutes, nb_comments,
        round(score_approves / total * 100.0, 2)::float4 as percent_approves,
        round(score_refutes / total * 100.0, 2)::float4 as percent_refutes,
        round(score_comments / total * 100.0, 2)::float4 as percent_comments
    from s
    order by title
    offset $* fetch first $* rows only
