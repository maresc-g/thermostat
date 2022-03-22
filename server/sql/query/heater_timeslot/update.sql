UPDATE heater_timeslot
   SET target_temperature = $2
     , start_day = $3
     , start_time = $4
     , end_day = $5
     , end_time = $6
 WHERE pk = $1
;