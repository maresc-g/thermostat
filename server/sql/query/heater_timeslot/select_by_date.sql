  WITH tmp AS (
        SELECT *
             , '1970-01-01'::date + start_day + start_time AS start_date
             , '1970-01-01'::date + end_day + end_time AS end_date
             , '1970-01-01'::date + $1::INTEGER + $2::TIME AS test_date
          FROM heater_timeslot
     )
SELECT *
  FROM tmp
 WHERE test_date BETWEEN start_date AND end_date
;
