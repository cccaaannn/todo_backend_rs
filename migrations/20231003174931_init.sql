CREATE TABLE "todo" (
	"id" UUID NOT NULL,
	"title" VARCHAR NOT NULL,
	"content" TEXT NOT NULL,
	"completed" BOOLEAN NOT NULL,
	"created_at" TIMESTAMP DEFAULT NOW(),
	"updated_at" TIMESTAMP DEFAULT NOW(),
	PRIMARY KEY ("id")
)
;
COMMENT ON COLUMN "todo"."id" IS '';
COMMENT ON COLUMN "todo"."title" IS '';
COMMENT ON COLUMN "todo"."content" IS '';
COMMENT ON COLUMN "todo"."completed" IS '';
COMMENT ON COLUMN "todo"."created_at" IS '';
COMMENT ON COLUMN "todo"."updated_at" IS '';

CREATE FUNCTION trigger_set_timestamp()
RETURNS TRIGGER AS $$
BEGIN
  NEW.updated_at = NOW();
RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER set_timestamp
BEFORE
UPDATE ON todo
FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();
