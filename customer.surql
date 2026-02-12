REMOVE TABLE IF EXISTS customer;

DEFINE TABLE customer TYPE NORMAL SCHEMAFULL;

-- name {
--    first   String
--    middle  String
--    last    [String] ?
-- }
DEFINE FIELD name                           ON TABLE customer TYPE object;
DEFINE FIELD name.first                     ON TABLE customer TYPE string;
DEFINE FIELD name.middle                    ON TABLE customer TYPE option<string>;
DEFINE FIELD name.last                      ON TABLE customer TYPE string;

-- deliver addr [] **contry specific?**
DEFINE FIELD deliver_address                ON TABLE customer TYPE array<object>;
DEFINE FIELD deliver_address.*.line1        ON TABLE customer TYPE string;
DEFINE FIELD deliver_address.*.line2        ON TABLE customer TYPE string;
DEFINE FIELD deliver_address.*.post_code    ON TABLE customer TYPE string;
DEFINE FIELD deliver_address.*.state        ON TABLE customer TYPE string;
DEFINE FIELD deliver_address.*.country      ON TABLE customer TYPE string;

-- billing details {
--      payment method (encrypt card details ?)
--      billing addr
-- }
DEFINE FIELD billing_details                ON TABLE customer TYPE array<object>;
DEFINE FIELD billing_details.*.name         ON TABLE customer TYPE string;
DEFINE FIELD billing_details.*.card         ON TABLE customer TYPE object;
DEFINE FIELD billing_details.*.card.type    ON TABLE customer TYPE string;
DEFINE FIELD billing_details.*.card.number  ON TABLE customer TYPE string;
DEFINE FIELD billing_details.*.card.expiry  ON TABLE customer TYPE string;
DEFINE FIELD billing_details.*.line1        ON TABLE customer TYPE string;
DEFINE FIELD billing_details.*.line2        ON TABLE customer TYPE string;
DEFINE FIELD billing_details.*.post_code    ON TABLE customer TYPE string;
DEFINE FIELD billing_details.*.state        ON TABLE customer TYPE string;
DEFINE FIELD billing_details.*.country      ON TABLE customer TYPE string;

-- purchase hist [<order>]
DEFINE FIELD purchase_history               ON TABLE customer TYPE array<record<order>>;

-- reviews [<review>]
DEFINE FIELD reviews                        ON TABLE customer TYPE array<record<review>>;

-- cart
DEFINE FIELD cart                           ON TABEL customer TYPE array<record<product>>;

-- wishlist ?
DEFINE FIELD wishlist                       ON TABEL customer TYPE array<record<product>>;


-- customer searches for item
--      buy now         -> purchase
--      add to cart     -> ...      -> purchase
--      add to wishlist -> ...      -> purchase

-- recomendations displayed on mainpage
--  based on purchase history

-- recomendations displayed on product page
--  based on current product


-- seperate address' ?
--  used in at least 3 places,
--      delivery
--      billing
--      store/warehouse