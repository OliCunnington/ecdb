REMOVE TABLE IF EXISTS review;

DEFINE TABLE review TYPE RELATION FROM customer TO product;

DEFINE FIELD order ON review TYPE record<order>;

DEFINE FIELD score          ON TABLE review TYPE int;
DEFINE FIELD comments       ON TABLE review TYPE string;
DEFINE FIELD creation_date  ON TABLE review VALUE time::now() READONLY;