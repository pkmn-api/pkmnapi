CREATE TABLE "users" (
    "id"                 VARCHAR NOT NULL,
    "date_create"        VARCHAR NOT NULL,
    "date_expire"        VARCHAR NOT NULL,
    "access_token_hash"  VARCHAR NOT NULL,
    "rom_id"             VARCHAR,
    PRIMARY KEY("id"),
    FOREIGN KEY("rom_id") REFERENCES "roms"("id")
);

CREATE UNIQUE INDEX "users_access_token_hash" ON "users" (
    "access_token_hash" ASC
);
