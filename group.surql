REMOVE TABLE IF EXISTS group;

DEFINE TABLE group TYPE NORMAL SCHEMAFULL;

DEFINE FIELD products ON TABLE group TYPE array<record<product>>;

-- variations?
-- list of fields that or values that differ? 
-- should probably link related products instead
-- use this as more of a "special offers/bundles" option