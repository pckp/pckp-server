CREATE SCHEMA pckp;

CREATE TABLE pckp.package (
    -- TODO: Add versions field (array of varchar)
    package_id serial PRIMARY KEY,
    package_name VARCHAR(255) UNIQUE NOT NULL,
    package_desc VARCHAR(65535),
    homepage VARCHAR(255),
    added_at INT8 NOT NULL,
    package_downloads INT4,
    versions VARCHAR(255) []
);

CREATE TABLE pckp.author (
    user_id serial PRIMARY KEY,
    user_name VARCHAR(255) UNIQUE NOT NULL,
    github_url VARCHAR(255),
    verified BOOLEAN,
    user_profile_img VARCHAR(255) -- Path to local fs that holds image
);

-- INSERT INTO pckp.package
-- VALUES (1, 'test', 'test', 'home', 1, 0, '{"0.1.0", "0.2.0"}');

-- SELECT * FROM pckp.package WHERE package_id = 1;
