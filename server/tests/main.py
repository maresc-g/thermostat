#!/usr/bin/env python3

import requests as req
import json

r = req.get('http://127.0.0.1:8080/v1/heater_timeslot')
print(f'{r.url} ({r.request.method}) => {r.status_code} : {r.json()}')

payload = {
    "target_temperature": 21.47,
    "start_day": 1,
    "start_time": "21:20:45",
    "end_day": 2,
    "end_time": "01:06:00",
}
r = req.post('http://127.0.0.1:8080/v1/heater_timeslot', data=json.dumps(payload))
print(f'{r.url} ({r.request.method}) => {r.status_code} : {r.text}')

r = req.get('http://127.0.0.1:8080/v1/heater_timeslot')
print(f'{r.url} ({r.request.method}) => {r.status_code} : {r.json()}')
pk = r.json()[0]["pk"]

payload = {
    "pk": pk,
    "target_temperature": 17.5,
    "start_day": 2,
    "start_time": "01:20:45",
    "end_day": 6,
    "end_time": "22:05:00",
}
r = req.put('http://127.0.0.1:8080/v1/heater_timeslot', data=json.dumps(payload))
print(f'{r.url} ({r.request.method}) => {r.status_code} : {r.text}')

r = req.get('http://127.0.0.1:8080/v1/heater_timeslot')
print(f'{r.url} ({r.request.method}) => {r.status_code} : {r.json()}')
