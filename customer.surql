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