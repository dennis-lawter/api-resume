create table skill_group (
    id integer primary key not null,
    group_name text not null,
    resume_id integer not null,
    foreign key (resume_id) references resume_overview(id)
);
