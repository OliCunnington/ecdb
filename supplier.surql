REMOVE TABLE IF EXISTS supplier;

DEFINE TABLE supplier TYPE NORMAL SCHEMAFULL;

DEFINE FIELD name           ON TABLE supplier TYPE string;
DEFINE FIELD min_volume     ON TABLE supplier TYPE int;
DEFINE FIELD max_volume     ON TABLE supplier TYPE int;
DEFINE FIELD address        ON TABLE supplier TYPE option<object>;

-- products (volume)
