-- customer "catalog" /main
SELECT 
    name, 
    price, 
    stock > 0 AS in_stock
    
    FROM product;
-- ORDER BY {filter selection}
-- LIMIT {page_max} or [n..n+page_max]

-- product
SELECT VALUE
    *
    FROM product
    WHERE $id = product.id
    LIMIT 1;

-- recommended based on current product categorys & ordered by average_rating
-- TODO filter same product "group/variant"
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

-- update stock on order creation
DEFINE EVENT order_submitted ON TABLE order
    WHEN $event = "CREATE"
    THEN {
        FOR $product IN $after.items {
            UPDATE $prodct.product SET stock -= $prodct.quantity;
        };
--        UPDATE $after.customer.open_orders SET VALUE $after.id;
    };

-- update stock on order cancellation
DEFINE EVENT order_submitted ON TABLE order
    WHEN $event = "UPDATE"
    THEN {
        IF $after.status = "Cancelled" {
            FOR $product IN $after.items {
                UPDATE $prodct.product SET stock += $prodct.quantity;
            };
        }
--      move order to purchase_history?
    };