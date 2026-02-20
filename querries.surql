-- customer "catalog" /main
SELECT 
    name, 
    price, 
    stock > 0 AS in_stock
    
    FROM product;
-- ORDER BY {filter selection}

-- product
SELECT VALUE
    *
    FROM product
    WHERE $id = product.id
    LIMIT 1;

SELECT
    name,
    price,
    stock > 0 as in_stock
FROM product
WHERE
    $id IS NOT product.id
    AND
    product.categorys CONTAINS $categorys
ORDER BY
    product.average_rating
DESC
LIMIT 3;