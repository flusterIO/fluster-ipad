{% for table_field in conundrum::ecosystem::db::tables::DatabaseTable::iter() %} 
{% if table_field.is_schemafull() %}
DEFINE TABLE {{table_field.to_string()}} SCHEMAFULL;
{% endif %}{% endfor %}
{% let join_map = crate::vector::database::joins::join_table::JoinTable::to_join_map() %}
{% for (k, v) in join_map.iter() %}
DEFINE TABLE {{k.to_string()}} SCHEMAFULL
  TYPE RELATION
  FROM {{v.0.to_string()}}
  TO {{v.1.to_string()}};
{% endfor %}
