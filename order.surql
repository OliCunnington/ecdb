REMOVE TABLE IF EXISTS order;
-- REMOVE TABLE IF EXISTS vendor_order;

DEFINE TABLE order TYPE RELATION FROM customer TO vendor;
DEFINE FIELD ordered_product ON order TYPE record<product>;

DEFINE FIELD destination                ON TABLE order TYPE object;
DEFINE FIELD destination.*.name         ON TABLE order TYPE string;
DEFINE FIELD destination.*.line1        ON TABLE order TYPE string;
DEFINE FIELD destination.*.line2        ON TABLE order TYPE string;
DEFINE FIELD destination.*.post_code    ON TABLE order TYPE string;
DEFINE FIELD destination.*.state        ON TABLE order TYPE string;
DEFINE FIELD destination.*.country      ON TABLE order TYPE string;

DEFINE FIELD items              ON TABLE order TYPE array<object>;
DEFINE FIELD item.*.product     ON TABLE order TYPE record<product>;
DEFINE FIELD item.*.quantity    ON TABLE order TYPE int;

-- actions? {cancel, schedule, track} ?
-- status {processing, shipped, complete}