<template>
  <div class="timeslot_day">
    <p>{{ day }}</p>
    <div class="wrapper">
      <div class="row" v-for="day in days" :key="day.id">
        <div class="time">{{ day.day }}</div>
        <ul>
          <li v-for="timeslot in day.timeslots" :key="timeslot.startTime">
            <p>
              {{ timeslot.startTime }} to {{ timeslot.endTime }} at {{ timeslot.temperature }}°C
            </p>
            <button>Remove timeslot</button>
          </li>
        </ul>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue'
import axios from 'axios'

export default defineComponent({
  data () {
    return {
      days: [] as {
        id: number
        day: string
        timeslots: {
          startTime: string
          endTime: string
          temperature: number
        }[]
      }[]
    }
  },
  created () {
    const days = ['Monday', 'Tuesday', 'Wednesday', 'Thursday', 'Friday', 'Saturday', 'Sunday']
    for (let i = 0; i < 6; i++) {
      this.days.push({ id: i, day: days[i], timeslots: [] })
    }
    axios
      .get('http://localhost:8080/v1/heater_timeslot')
      .then(response => {
        console.log(response)
        for (const timeslot of response.data) {
          this.days[timeslot.day].timeslots.push({ startTime: timeslot.start_time, endTime: timeslot.end_time, temperature: timeslot.target_temperature })
        }
      })
  }
  // },
  // methods: {
  //   addTimeSlot (event) {
  //     console.log('Hello ' + event.target)
  //   }
  // }
})
</script>

<style lang="scss">
.wrapper {
  display: grid;
}
.row {
  text-align: left;
}
</style>
