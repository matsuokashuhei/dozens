CREATE TABLE "user_relationships" (
    "id" UUID NOT NULL,
    "follower_id" UUID NOT NULL,
    "followee_id" UUID NOT NULL,
    "created_at" TIMESTAMPTZ(6) NOT NULL,
    PRIMARY KEY ("id")
);
CREATE UNIQUE INDEX "index_user_relationships_by_follower_id_and_followee_id" ON "user_relationships" ("follower_id", "followee_id");
CREATE INDEX "index_user_relationships_by_follower_id" ON "user_relationships" ("follower_id");
CREATE INDEX "index_user_relationships_by_followee_id" ON "user_relationships" ("followee_id");
