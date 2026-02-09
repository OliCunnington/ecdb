surreal import -u root -p root --ns main --db ecdb ./customer.surql
wait 1

surreal import -u root -p root --ns main --db ecdb ./vendor.surql
wait 1

surreal import -u root -p root --ns main --db ecdb ./product.surql
wait 1

surreal import -u root -p root --ns main --db ecdb ./order.surql
