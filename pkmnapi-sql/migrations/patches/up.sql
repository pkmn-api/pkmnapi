CREATE TABLE "patches" (
    id      VARCHAR NOT NULL,
    user_id VARCHAR NOT NULL,
    data    BLOB NOT NULL,
    PRIMARY KEY("id"),
    FOREIGN KEY("user_id") REFERENCES "users"("id")
);
