REMOVE TABLE IF EXISTS product;

DEFINE TABLE product TYPE NORMAL SCHEMAFULL;

DEFINE FIELD name           ON TABLE product TYPE string;
DEFINE FIELD stock          ON TABLE product TYPE int;
DEFINE FIELD price          ON TABLE product TYPE float;
DEFINE FIELD categorys      ON TABLE product TYPE array<string>;
DEFINE FIELD reviews        ON TABLE product TYPE array<record<review>>;
DEFINE FIELD average_rating ON TABLE product VALUE <future> { 
    math::sum( SELECT VALUE score FROM review WHERE product_id = $id / SELECT count() FROM review WHERE product_id = $id )
};
DEFINE FIELD description    ON TABLE product TYPE string;


