REMOVE TABLE IF EXISTS reviews;

DEFINE TABLE review TYPE RELATION FROM customer TO product;
DEFINE FIELD order ON review TYPE record<order>;