REMOVE TABLE IF EXISTS order;

DEFINE TABLE order TYPE RELATION FROM customer TO vendor;

DEFINE FIELD customer                   ON TABLE order TYPE record<customer>;

DEFINE FIELD destination                ON TABLE order TYPE object;
DEFINE FIELD destination.*.name         ON TABLE order TYPE string;
DEFINE FIELD destination.*.line1        ON TABLE order TYPE string;
DEFINE FIELD destination.*.line2        ON TABLE order TYPE string;
DEFINE FIELD destination.*.post_code    ON TABLE order TYPE string;
DEFINE FIELD destination.*.state        ON TABLE order TYPE string;
DEFINE FIELD destination.*.country      ON TABLE order TYPE string;

DEFINE FIELD items                      ON TABLE order TYPE array<object>;
DEFINE FIELD item.*.product             ON TABLE order TYPE record<product>;
DEFINE FIELD item.*.quantity            ON TABLE order TYPE int;

-- status {processing, shipped, complete}
DEFINE FIELD status                     ON TABLE order TYPE string ASSERT $value IN $ORDER_STATUS;

DEFINE FIELD order_date                 ON TABLE order TYPE datetime;

-- actions? {cancel, schedule, track} ?
-- issue / complaint
-- option relation?
--   open 'ticket' if none through web?