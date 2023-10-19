create table previous_position (
    id integer primary key not null,
    employer text not null,
    title text not null,
    employment_start_date text not null,
    employment_end_date text not null,
    resume_id integer not null,
    foreign key (resume_id) references resume_overview(id)
);
