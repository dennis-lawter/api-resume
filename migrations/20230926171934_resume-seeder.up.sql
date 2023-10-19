/* resume_overview */
insert into resume_overview (id, full_name, title, objective)
values (1, "Dennis Lawter", "Software Engineer", "I have always been driven by the desire to solve complex problems and stay up-to-date with the latest technologies and methodologies. My experience in developing, deploying, and maintaining software has allowed me to make a meaningful impact in the field, and I am constantly seeking new opportunities to push my skills to the next level.");

/* contact_info */
insert into contact_info (id, method, information, resume_id)
values (1, "e-mail", "dennis.lawter@gmail.com", 1);

insert into contact_info (id, method, information, resume_id)
values (2, "Phone (mobile)", "(864) 606-4606", 1);

insert into contact_info (id, method, information, resume_id)
values (3, "GitHub", "https://www.github.com/dennis-lawter/", 1);

insert into contact_info (id, method, information, resume_id)
values (4, "LinkedIn", "https://www.linkedin.com/in/dennislawter/", 1);

/* skills */
insert into skill_group (id, group_name, resume_id)
values (1, "Programming Languages", 1);
    insert into skill (skill_group_id, skill_name)
    values
        (1, "PHP (5.x, 7.x, 8.x)"),
        (1, "Rust (2021)"),
        (1, "Python (3.x)"),
        (1, "C++ (C++98, C++03, C++11, C++14, C++17)"),
        (1, "C (C99, C17)"),
        (1, "Java (JDK 8 and JDK 17)"),
        (1, "Automation Scripting (Bash, Makefile, Dockerfile)"),
        (1, "SQL (MySQL, T-SQL, PostgreSQL, SQLite)"),
        (1, "Javascript (ECMAScript 2009, 2015, 2016, 2017)");

insert into skill_group (id, group_name, resume_id)
values (2, "Development Methodologies", 1);
    insert into skill (skill_group_id, skill_name)
    values
        (2, "Agile Methodology"),
        (2, "Scrum"),
        (2, "REST"),
        (2, "CI/CD"),
        (2, "Object Oriented Design"),
        (2, "Functional Programming");

insert into skill_group (id, group_name, resume_id)
values (3, "Operating Systems", 1);
    insert into skill (skill_group_id, skill_name)
    values
        (3, "Debian Linux"),
        (3, "FreeBSD"),
        (3, "Microsoft Windows & Windows Server");

insert into skill_group (id, group_name, resume_id)
values (4, "Project Management", 1);
    insert into skill (skill_group_id, skill_name)
    values
        (4, "VCS (Git, Subversion)"),
        (4, "Build Systems (Composer, Cargo, CMake, Maven)"),
        (4, "CI/CD (GitHub Actions, Jenkins, Groovy)"),
        (4, "Collaboration (Jira, Confluence, Slack)"),
        (4, "Containerization (Docker, DockerCompose, Kubernetes)");

insert into skill_group (id, group_name, resume_id)
values (5, "Frameworks and SDKs", 1);
    insert into skill (skill_group_id, skill_name)
    values
        (5, "PHP (Laravel, Yii 2, Yii 1.1)"),
        (5, "Rust (Poem, Actix, Rocket, Tide, Tauri)"),
        (5, "Python (Django)"),
        (5, "C / C++ (SDL, SFML, OpenGL)"),
        (5, "Javascript (jQuery, Bootstrap, React)"),
        (5, "Other (SOAP, Godot)");

insert into skill_group (id, group_name, resume_id)
values (6, "Misc", 1);
    insert into skill (skill_group_id, skill_name)
    values
        (6, "Accounting Packages (SAP, Epicor)"),
        (6, "Web Hosting (Nginx, Apache)"),
        (6, "Server Monitoring (Grafana, Prometheus)");

/* positions */
insert into previous_position (id, employer, title, employment_start_date, employment_end_date, resume_id)
values (
    1,
    "University of South Carolina",
    "Computer Science Tutor",
    "August 2010",
    "May 2011",
    1
);
insert into previous_position_achievement (previous_position_id, achievement)
values
    (1, "Helped students understand computer science topics in a one-on-one environment."),
    (1, "Learned various programming languages for the needs of my fellow students."),
    (1, "Taught the basics behind algorithm design, data structures, and artificial intelligence.");

insert into previous_position (id, employer, title, employment_start_date, employment_end_date, resume_id)
values (
    2,
    "Servosity",
    "Web Developer",
    "November 2011",
    "December 2011",
    1
);
insert into previous_position_achievement (previous_position_id, achievement)
values
    (2, "Managed deployments to a production environment."),
    (2, "Created new API integrations on a well designed system."),
    (2, "Gained experience with version control in a professional environment.");

insert into previous_position (id, employer, title, employment_start_date, employment_end_date, resume_id)
values (
    3,
    "Ingram Micro",
    "Software Engineer",
    "June 2013",
    "March 2022",
    1
);
insert into previous_position_achievement (previous_position_id, achievement)
values
    (3, "Introduced integration with Epicor and SAP, connecting an existing backend to external inventory and accounting management packages."),
    (3, "Automated QA and production deployment pipelines with many installed servers and cloud environments."),
    (3, "Worked on the conversion of a legacy PHP application to the Yii 1.1 framework, introducing ORM and MVC patterns."),
    (3, "Maintained multiple Debian Linux VMs and AWS instances running LAMPP-stack servers, Kubernetes clusters, SFTP hosts, and Jenkins automation servers."),
    (3, "Developed a REST API to leverage existing features externally, providing a pathway for modernization and integration with other systems.");

insert into previous_position (id, employer, title, employment_start_date, employment_end_date, resume_id)
values (
    4,
    "Inseego",
    "Senior Software Engineer",
    "March 2022",
    "Present",
    1
);
insert into previous_position_achievement (previous_position_id, achievement)
values
    (4, "Modernized a monolithic software stack using microservices, architecting REST APIs to replace direct database integrations."),
    (4, "Created several private installable modules to be deployed using composer, pushing for consistency and reducing code reuse."),
    (4, "Redesigned portions of the billing system for greater accuracy and auditing visiblity.");

/* education */
insert into education (id, school, degree, education_start_date, education_end_date, resume_id)
values (1, "University of South Carolina", "Bachelor of Computer Science", "August 2007", "May 2011", 1);
