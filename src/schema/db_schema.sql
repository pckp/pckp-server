CREATE SCHEMA pckp;

CREATE TABLE pckp.package (
    id SERIAL PRIMARY KEY,
    author_id INT NOT NULL,
    name VARCHAR(255) UNIQUE NOT NULL,
    repo_name VARCHAR(255),
    homepage VARCHAR(255),
    added_timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    downloads INT4 DEFAULT 0,
);

CREATE TABLE pckp.author (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) UNIQUE NOT NULL,
    github_user VARCHAR(255),
    verified BOOLEAN,
    profile_image VARCHAR(255) -- url
);
