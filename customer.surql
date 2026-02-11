REMOVE TABLE IF EXISTS customer;

DEFINE TABLE customer TYPE NORMAL SCHEMAFULL;

-- name {
    first   String
    middle  String
    last    [String]
}
-- deliver addr []
-- billing details {
--  payment methods []
--  billing addr []
-- }
-- purchase hist [<order>]
-- reviews [<review>]
-- cart
-- wishlist ?


-- customer searches for item
--      buy now         -> purchase
--      add to cart     -> ...      -> purchase
--      add to wishlist -> ...      -> purchase

-- recomendations
--  based on purchase history
--  based on current product
