SELECT *
  FROM heater_timeslot
 WHERE day = $1
   AND $2::TIME BETWEEN start_time AND end_time
;
