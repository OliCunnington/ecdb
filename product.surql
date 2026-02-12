REMOVE TABLE IF EXISTS product;

DEFINE TABLE product TYPE NORMAL SCHEMAFULL;

-- stock
--      min stock ? -> notify for re-supply?
DEFINE FIELD stock          ON TABLE product TYPE int;

-- price
--  product group:
--      variants ? eg. [100g, 250g, 500g, ...], [1, 3, 5, ...] etc 
DEFINE FIELD price          ON TABLE product TYPE float;

-- categorys []
DEFINE FIELD categorys      ON TABLE product TYPE array<string>;

-- reviews [] (LIMIT 3, SORT date, score asc, score desc)
DEFINE FIELD reviews        ON TABLE product TYPE array<record<review>>;

-- avg
DEFINE FIELD average_rating ON TABLE product VALUE <future> { 
    math::sum( SELECT VALUE score FROM review where product_id = $id / SELECT count() FROM review where product_id = $id )
};

-- description
--   full text searches ?
DEFINE FIELD description    ON TABLE product TYPE string;


