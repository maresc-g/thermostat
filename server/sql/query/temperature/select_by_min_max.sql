SELECT *
  FROM temperature_history
 WHERE date BETWEEN $1 AND $2
;