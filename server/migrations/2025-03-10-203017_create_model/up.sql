-- create tables
CREATE TABLE users (
    id serial PRIMARY KEY,
    username text NOT NULL
);

CREATE TABLE blogs (
    id serial PRIMARY KEY,
    user_id integer NOT NULL REFERENCES users(id),
    create_date timestamptz NOT NULL,
    updated_date timestamptz NOT NULL,
    title varchar NOT NULL,
    description varchar NOT NULL
);

CREATE TABLE posts (
    id serial PRIMARY KEY,
    user_id integer NOT NULL REFERENCES users(id),
    blog_id integer NOT NULL REFERENCES blogs(id),
    create_date timestamptz NOT NULL,
    updated_date timestamptz NOT NULL,
    published_date timestamptz,
    title varchar NOT NULL,
    description varchar NOT NULL,
    content text NOT NULL
);
