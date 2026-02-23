DROP TABLE IF EXISTS group;

DEFINE TABLE group TYPE NORMAL SCHEMAFULL;

DEFINE FIELD products ON TABLE group TYPE array<record<prodct>>;

-- variations?
-- list of fields that or values that differ? 