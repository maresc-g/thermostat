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
            <button @click="removeTimeSlot(day.id, timeslot.pk)">Remove timeslot</button>
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
    addTimeSlot (day: string, startHour: string, startMinutes: string, endHour: string, endMinutes: string, temperature: number) {
      const days = ['Monday', 'Tuesday', 'Wednesday', 'Thursday', 'Friday', 'Saturday', 'Sunday']
      this.showModal = false
      const start = startHour + ':' + startMinutes + ':00'
      const end = endHour + ':' + endMinutes + ':00'
      console.log('Adding timeslot for ' + day + ' from ' + start + ' to ' + end)
      axios
        .post('http://localhost:8080/v1/heater_timeslot', { day: days.indexOf(day), target_temperature: temperature, start_time: start, end_time: end })
        .then(response => {
          console.log(response)
          this.days[response.data.day].timeslots.push({ pk: response.data.pk, startTime: response.data.start_time, endTime: response.data.end_time, temperature: response.data.target_temperature })
        })
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

.modal-mask {
  position: fixed;
  z-index: 9998;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background-color: rgba(0, 0, 0, 0.5);
  display: table;
  transition: opacity 0.3s ease;
}

.modal-wrapper {
  display: table-cell;
  vertical-align: middle;
}

.modal-container {
  width: 300px;
  margin: 0px auto;
  padding: 20px 30px;
  background-color: #fff;
  border-radius: 2px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.33);
  transition: all 0.3s ease;
  font-family: Helvetica, Arial, sans-serif;
}

.modal-header h3 {
  margin-top: 0;
  color: #42b983;
}

.modal-body {
  margin: 20px 0;
}

.modal-default-button {
  float: right;
}

/*
 * The following styles are auto-applied to elements with
 * transition="modal" when their visibility is toggled
 * by Vue.js.
 *
 * You can easily play with the modal transition by editing
 * these styles.
 */

.modal-enter {
  opacity: 0;
}

.modal-leave-active {
  opacity: 0;
}

.modal-enter .modal-container,
.modal-leave-active .modal-container {
  -webkit-transform: scale(1.1);
  transform: scale(1.1);
}
</style>
