-- Your SQL goes here
CREATE TABLE characters (
    id integer PRIMARY KEY,
    first_name VARCHAR,
    diplomacy integer,
    martial integer,
    stewardship integer,
    intrigue integer,
    learning integer,
    prowess integer,
    faith_id INTEGER REFERENCES faiths(id),
    house_id INTEGER REFERENCES houses(id)
);

CREATE TABLE faiths (
    id INTEGER PRIMARY KEY,
    tag VARCHAR
);

CREATE TABLE houses (
                        id INTEGER PRIMARY KEY,
                        name VARCHAR
);