CREATE TABLE "rom_data" (
    "id"   VARCHAR NOT NULL,
    "name" VARCHAR NOT NULL,
    "data" BYTEA NOT NULL,
    PRIMARY KEY("id")
);

CREATE TABLE "roms" (
    "id"          VARCHAR NOT NULL,
    "date_create" VARCHAR NOT NULL,
    "name"        VARCHAR NOT NULL,
    "etag"        VARCHAR NOT NULL,
    "rom_data_id" VARCHAR NOT NULL,
    FOREIGN KEY("rom_data_id") REFERENCES "rom_data"("id"),
    PRIMARY KEY("id")
);

CREATE TABLE "savs" (
    "id"          VARCHAR NOT NULL,
    "date_create" VARCHAR NOT NULL,
    "data"        BYTEA NOT NULL,
    "etag"        VARCHAR NOT NULL,
    PRIMARY KEY("id")
);

CREATE TABLE "users" (
    "id"                 VARCHAR NOT NULL,
    "date_create"        VARCHAR NOT NULL,
    "date_expire"        VARCHAR NOT NULL,
    "access_token_hash"  VARCHAR NOT NULL,
    "delete_code"        VARCHAR,
    "rom_id"             VARCHAR,
    "sav_id"             VARCHAR,
    PRIMARY KEY("id"),
    FOREIGN KEY("rom_id") REFERENCES "roms"("id"),
    FOREIGN KEY("sav_id") REFERENCES "savs"("id")
);

CREATE UNIQUE INDEX "users_access_token_hash" ON "users" (
    "access_token_hash" ASC
);

CREATE TABLE "rom_patches" (
    "id"          VARCHAR NOT NULL,
    "date_create" VARCHAR NOT NULL,
    "user_id"     VARCHAR NOT NULL,
    "data"        BYTEA NOT NULL,
    "description" VARCHAR,
    "etag"        VARCHAR NOT NULL,
    PRIMARY KEY("id"),
    FOREIGN KEY("user_id") REFERENCES "users"("id")
);

CREATE TABLE "sav_patches" (
    "id"          VARCHAR NOT NULL,
    "date_create" VARCHAR NOT NULL,
    "user_id"     VARCHAR NOT NULL,
    "data"        BYTEA NOT NULL,
    "description" VARCHAR,
    "etag"        VARCHAR NOT NULL,
    PRIMARY KEY("id"),
    FOREIGN KEY("user_id") REFERENCES "users"("id")
);
