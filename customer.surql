REMOVE TABLE IF EXISTS customer;

DEFINE TABLE customer TYPE NORMAL SCHEMAFULL;

DEFINE FIELD name                           ON TABLE customer TYPE object;
DEFINE FIELD name.first                     ON TABLE customer TYPE string;
DEFINE FIELD name.middle                    ON TABLE customer TYPE option<string>;
DEFINE FIELD name.last                      ON TABLE customer TYPE string;

DEFINE FIELD delivery_address               ON TABLE customer TYPE option<array<object>>;
DEFINE FIELD delivery_address.*.line1       ON TABLE customer TYPE string;
DEFINE FIELD delivery_address.*.line2       ON TABLE customer TYPE option<string>;
DEFINE FIELD delivery_address.*.post_code   ON TABLE customer TYPE string;
DEFINE FIELD delivery_address.*.state       ON TABLE customer TYPE string;
DEFINE FIELD delivery_address.*.country     ON TABLE customer TYPE string;

DEFINE FIELD billing_details                ON TABLE customer TYPE option<array<object>>;
DEFINE FIELD billing_details.*.name         ON TABLE customer TYPE string;
DEFINE FIELD billing_details.*.card         ON TABLE customer TYPE object;
DEFINE FIELD billing_details.*.card.type    ON TABLE customer TYPE string;
DEFINE FIELD billing_details.*.card.number  ON TABLE customer TYPE string;
DEFINE FIELD billing_details.*.card.expiry  ON TABLE customer TYPE string;
DEFINE FIELD billing_details.*.line1        ON TABLE customer TYPE string;
DEFINE FIELD billing_details.*.line2        ON TABLE customer TYPE option<string>;
DEFINE FIELD billing_details.*.post_code    ON TABLE customer TYPE string;
DEFINE FIELD billing_details.*.state        ON TABLE customer TYPE string;
DEFINE FIELD billing_details.*.country      ON TABLE customer TYPE string;

DEFINE FIELD cart                           ON TABLE customer TYPE array<record<product>>;
DEFINE FIELD wishlist                       ON TABLE customer TYPE array<record<product>>;

DEFINE FIELD purchase_history               ON TABLE customer VALUE <future> {
    SELECT order_date, items FROM order WHERE customer = $id AND status = "Completed";
}; -- array<record<order>>;
DEFINE FIELD open_orders                    ON TABLE customer VALUE <future> {
    SELECT order_date, items FROM order WHERE customer = $id AND status IN ["Processing", "Shipped"];
}; -- array<record<order>>;
DEFINE FIELD reviews                        ON TABLE customer VALUE <future> {
    SELECT creation_date, score, order FROM review WHERE customer = $id;
}; --TYPE array<record<review>>;
