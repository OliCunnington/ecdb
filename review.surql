REMOVE TABLE IF EXISTS reviews;

DEFINE TABLE review TYPE RELATION FROM customer TO product;
DEFINE FIELD order ON review TYPE record<order>;

-- score
DEFINE FIELD score          ON TABLE review TYPE int;

-- comments
DEFINE FIELD comments       ON TABLE review TYPE string;

-- date
DEFINE FIELD creation_date  ON_TABLE review VALUE time::now() READONLY;