<template>
  <div class="timeslot_day">
    <div class="wrapper">
      <div class="row" v-for="day in days" :key="day.id">
        <div class="time">{{ day.day }}</div>
        <ul>
          <li v-for="timeslot in day.timeslots" :key="timeslot.startTime">
            <p>
              {{ timeslot.startTime }} to {{ timeslot.endTime }} at {{ timeslot.temperature }}°C
            </p>
            <button class="remove-button" @click="removeTimeSlot(day.id, timeslot.pk)">Remove timeslot</button>
          </li>
        </ul>
      </div>
      <div class="row">
        <button id="show-modal" @click="showModal = true">Add timeslot</button>
        <AddTimeSlot v-if="showModal" @addTimeSlot="addTimeSlot" @cancel="showModal = false"/>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue'
import axios from 'axios'
import AddTimeSlot from '@/components/timeslot/AddTimeSlot.vue'

export default defineComponent({
  data () {
    return {
      showModal: false,
      days: [] as {
        id: number
        day: string
        timeslots: {
          pk: number
          startTime: string
          endTime: string
          temperature: number
        }[]
      }[]
    }
  },
  created () {
    const days = ['Monday', 'Tuesday', 'Wednesday', 'Thursday', 'Friday', 'Saturday', 'Sunday']
    for (let i = 0; i < 7; i++) {
      this.days.push({ id: i, day: days[i], timeslots: [] })
    }
    axios
      .get('http://localhost:8080/v1/heater_timeslot')
      .then(response => {
        console.log(response)
        for (const timeslot of response.data) {
          this.days[timeslot.day].timeslots.push({ pk: timeslot.pk, startTime: timeslot.start_time, endTime: timeslot.end_time, temperature: timeslot.target_temperature })
        }
      })
  },
  methods: {
    addTimeSlot (days_selected: [string], start_time: { [key: string]: number }, end_time: { [key: string]: number }, temperature: number) {
      const days = ['Monday', 'Tuesday', 'Wednesday', 'Thursday', 'Friday', 'Saturday', 'Sunday']
      this.showModal = false
      const start = start_time['hours'] + ':' + start_time['minutes'] + ':00'
      const end = end_time['hours'] + ':' + end_time['minutes'] + ':00'
      for (let day of days_selected) {
        console.log('Adding timeslot for ' + day + ' from ' + start + ' to ' + end)
        axios
          .post('http://localhost:8080/v1/heater_timeslot', { day: days.indexOf(day), target_temperature: temperature, start_time: start, end_time: end })
          .then(response => {
            console.log(response)
            this.days[response.data.day].timeslots.push({ pk: response.data.pk, startTime: response.data.start_time, endTime: response.data.end_time, temperature: response.data.target_temperature })
          })
      }
    },
    removeTimeSlot (day: number, pk: number) {
      axios
        .delete('http://localhost:8080/v1/heater_timeslot/' + pk)
        .then(response => {
          console.log(response)
          this.days[day].timeslots = this.days[day].timeslots.filter(timeslot => timeslot.pk !== pk)
        })
    }
  },
  components: { AddTimeSlot }
})
</script>

<style lang="scss">
.wrapper {
  display: grid;
}
.row {
  text-align: left;
}

.remove-button {
  background-color: #d61e1e;
}

</style>
