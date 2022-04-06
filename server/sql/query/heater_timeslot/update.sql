UPDATE heater_timeslot
   SET target_temperature = $2
     , day = $3
     , start_time = $4
     , end_time = $5
 WHERE pk = $1
;