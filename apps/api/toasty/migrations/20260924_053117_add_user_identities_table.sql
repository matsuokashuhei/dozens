CREATE TABLE "user_identities" (
    "id" UUID NOT NULL,
    "user_id" UUID NOT NULL,
    "iss" TEXT NOT NULL,
    "sub" TEXT NOT NULL,
    PRIMARY KEY ("id")
);
CREATE UNIQUE INDEX "index_user_identities_by_iss_and_sub" ON "user_identities" ("iss", "sub");
CREATE INDEX "index_user_identities_by_user_id" ON "user_identities" ("user_id");
