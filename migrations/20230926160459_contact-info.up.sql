create table contact_info (
    id integer primary key not null,
    method text not null,
    information text not null,
    resume_id integer not null,
    foreign key (resume_id) references resume_overview(id)
);
