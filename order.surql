REMOVE TABLE IF EXISTS order;
-- REMOVE TABLE IF EXISTS vendor_order;

DEFINE TABLE order TYPE RELATION FROM customer TO vendor;
DEFINE FIELD ordered_product ON order TYPE record<product>;

-- status {processing, shipped, complete}
-- delivery address

-- actions? {cancel, schedule, track} ?

-- multiple items ?
-- quantity

-- order {
--     prod1 x 3,
--     prod2 x 1,
--     prod3 x 2
-- }