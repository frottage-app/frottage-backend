CREATE TABLE prompt (
    prompt_id integer primary key,
    prompt text unique not null
);

CREATE TABLE target (
    target_name text primary key,
    aspect_ratio text not null,
    CHECK (
        aspect_ratio like '%:%'
        and cast(
            cast(
                substr (aspect_ratio, 1, instr (aspect_ratio, ':') -1) as integer
            ) as text
        ) = substr (aspect_ratio, 1, instr (aspect_ratio, ':') -1)
        and cast(
            cast(
                substr (aspect_ratio, instr (aspect_ratio, ':') + 1) as integer
            ) as text
        ) = substr (aspect_ratio, instr (aspect_ratio, ':') + 1)
    )
);

CREATE TABLE prompt_target (
    prompt_id integer not null,
    target_name text not null,
    foreign key (prompt_id) references prompt (prompt_id) on delete cascade on update cascade,
    foreign key (target_name) references target (target_name) on delete cascade on update cascade
);

CREATE TABLE vote (
    prompt_id integer not null,
    target_name text not null,
    stars integer not null,
    vote_value real generated always as ((cast(stars as real) - 1) / 4) virtual,
    created_at integer not null default (unixepoch ('subsec') * 1000),
    foreign key (prompt_id) references prompt (prompt_id) on delete cascade on update cascade,
    foreign key (target_name) references target (target_name) on delete cascade on update cascade,
    CHECK (
        stars >= 1
        AND stars <= 5
    )
);

create table image (
    prompt_id integer not null,
    target_name text not null,
    image_url text not null,
    created_at integer not null default (unixepoch ('subsec') * 1000),
    foreign key (prompt_id) references prompt (prompt_id) on delete cascade on update cascade,
    foreign key (target_name) references target (target_name) on delete cascade on update cascade
);
