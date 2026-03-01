surreal import -u root -p root --ns main --db ecdb ./customer.surql
wait 1

surreal import -u root -p root --ns main --db ecdb ./vendor.surql
wait 1

surreal import -u root -p root --ns main --db ecdb ./supplier.surql
wait 1

surreal import -u root -p root --ns main --db ecdb ./product.surql
wait 1

surreal import -u root -p root --ns main --db ecdb ./group.surql
wait 1

surreal import -u root -p root --ns main --db ecdb ./params.surql
wait 1

surreal import -u root -p root --ns main --db ecdb ./order.surql
wait 1

surreal import -u root -p root --ns main --db ecdb ./review.surql
wait 1

surreal import -u root -p root --ns main --db ecdb ./data/customers.surql
wait 1

surreal import -u root -p root --ns main --db ecdb ./data/suppliers.surql
wait 1

surreal import -u root -p root --ns main --db ecdb ./data/products.surql
wait 1

