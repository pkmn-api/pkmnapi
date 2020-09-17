CREATE TABLE "sav_patches" (
    "id"          VARCHAR NOT NULL,
    "date_create" VARCHAR NOT NULL,
    "user_id"     VARCHAR NOT NULL,
    "data"        BLOB NOT NULL,
    "description" VARCHAR,
    PRIMARY KEY("id"),
    FOREIGN KEY("user_id") REFERENCES "users"("id")
);
