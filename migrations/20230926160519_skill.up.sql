create table skill (
    id integer primary key not null,
    skill_group_id integer not null,
    skill_name text not null,
    foreign key (skill_group_id) references skill_group(id)
);
