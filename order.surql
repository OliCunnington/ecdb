REMOVE TABLE IF EXISTS order;
-- REMOVE TABLE IF EXISTS vendor_order;

DEFINE TABLE order TYPE RELATION FROM customer TO vendor;
DEFINE FIELD ordered_product ON order TYPE record<product>;