REMOVE TABLE IF EXISTS product;

DEFINE TABLE product TYPE NORMAL SCHEMAFULL;

-- stock
--      min stock ? -> notify for re-supply?

-- price
--  product group:
--      variants ? eg. [100g, 250g, 500g, ...], [1, 3, 5, ...] etc 

-- categorys []

-- reviews [] (LIMIT 3, SORT date, score asc, score desc)

-- avg
-- <future> sum(SELECT VALUE score FROM REVIEW where product_id = $id)/len(SELECT VALUE score FROM REVIEW where product_id = $id);
-- len can probs be replaced with a count fnc

-- description
--   full text searches ?


