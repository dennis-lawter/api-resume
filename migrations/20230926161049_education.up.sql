create table education(
    id integer primary key not null,
    school text not null,
    degree text not null,
    education_start_date text not null,
    education_end_date text not null,
    resume_id integer not null,
    foreign key (resume_id) references resume_overview(id)
);
