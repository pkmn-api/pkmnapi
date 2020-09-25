CREATE TABLE "roms" (
    "id"          VARCHAR NOT NULL,
    "date_create" VARCHAR NOT NULL,
    "name"        VARCHAR NOT NULL,
    "etag"        VARCHAR NOT NULL,
    "rom_data_id" VARCHAR NOT NULL,
    FOREIGN KEY("rom_data_id") REFERENCES "rom_data"("id"),
    PRIMARY KEY("id")
);
