-- create tables
CREATE TABLE posts (
    id serial PRIMARY KEY,
    user_id integer NOT NULL,
    blog_id integer NOT NULL,
    create_date timestamptz NOT NULL,
    updated_date timestamptz NOT NULL,
    published_date timestamptz,
    title varchar NOT NULL,
    description varchar NOT NULL,
    content text NOT NULL
);

CREATE TABLE users (
    id serial PRIMARY KEY,
    username text NOT NULL
);

CREATE TABLE blogs (
    id serial PRIMARY KEY,
    user_id integer NOT NULL,
    create_date timestamptz NOT NULL,
    updated_date timestamptz NOT NULL,
    title varchar NOT NULL,
    description varchar NOT NULL
);
