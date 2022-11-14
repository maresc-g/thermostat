<template>
    <transition name="modal">
        <div class="modal-mask">
            <div class="modal-wrapper">
                <div class="modal-container">
                    <div class="modal-header">
                        <h3>Add a timeslot</h3>
                    </div>

                    <div class="modal-body">
                      <label for="day-select" class="label-day">Day : </label>
                      <select name="days" class="select-day" id="day-select" v-model="day_selected">
                        <option v-for="day in days" :key="day.id">{{ day.name }}</option>
                      </select>

                      <label for="start_hour-select" class="label-start">Start Time : </label>
                      <select name="hours" class="select-start-hour" id="start_hour-select" v-model="start_hour_selected">
                        <option v-for="hour in hours" :key="hour.id">{{ hour.value }}</option>
                      </select>
                      <select name="minutes" class="select-start-minute" id="start_minute-select" v-model="start_minute_selected">
                        <option v-for="minute in minutes" :key="minute.id">{{ minute.value }}</option>
                      </select>

                      <label for="end_hour-select" class="label-end">End Time : </label>
                      <select name="hours" class="select-end-hour" id="end_hour-select" v-model="end_hour_selected">
                        <option v-for="hour in hours" :key="hour.id">{{ hour.value }}</option>
                      </select>
                      <select name="minutes" class="select-end-minute" id="end_minute-select" v-model="end_minute_selected">
                        <option v-for="minute in minutes" :key="minute.id">{{ minute.value }}</option>
                      </select>

                      <label for="temperature-input" class="label-temperature">Temperature :</label>
                      <Slider v-model="temperature_selected" :min=160 :max=260 :step=5 class="temperature-input" :format="format" tooltipPosition="bottom"/>

                    </div>
                    <div class="modal-footer">
                      <button class="modal-default-button" @click="$emit('cancel')">
                          Cancel
                      </button>
                      <button class="modal-default-button" @click="$emit('addTimeSlot', day_selected, start_hour_selected, start_minute_selected, end_hour_selected, end_minute_selected, temperature_selected)">
                          OK
                      </button>
                    </div>
                </div>
            </div>
        </div>
    </transition>
</template>

<script lang="ts">
import { defineComponent } from 'vue'
import Slider from '@vueform/slider'

export default defineComponent({
  emits: ['addTimeSlot', 'cancel'],
  components: { Slider },
  data () {
    return {
      days: [
        { id: 0, name: 'Monday' },
        { id: 1, name: 'Tuesday' },
        { id: 2, name: 'Wednesday' },
        { id: 3, name: 'Thursday' },
        { id: 4, name: 'Friday' },
        { id: 5, name: 'Saturday' },
        { id: 6, name: 'Sunday' }
      ],
      hours: [] as { id: number, value: string }[],
      minutes: [] as { id: number, value: string }[],
      day_selected: '',
      start_hour_selected: '',
      start_minute_selected: '',
      end_hour_selected: '',
      end_minute_selected: '',
      temperature_selected: 200.0,
      format: function (temperature_selected: number) {
        return temperature_selected/10
      }
    }
  },
  created () {
    for (let i = 0; i < 24; i++) {
      this.hours.push({ id: i, value: i.toString().padStart(2, '0') })
    }
    for (let i = 0; i < 60; i += 5) {
      this.minutes.push({ id: i, value: i.toString().padStart(2, '0') })
    }
  }
})
</script>

<style src="@vueform/slider/themes/default.css"></style>

<style lang="scss">
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
  width: 800px;
  height: 200px;
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
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  grid-template-rows: repeat(3, 1fr);
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

.label-day {
  grid-column: 1 / 1;
  grid-row: 1 / 1;
}
.select-day {
  grid-column: 2 / span 3;
  grid-row: 1 / 1;
}
.label-start {
  grid-column: 1 / 1;
  grid-row: 2 / 2;
}
.select-start-hour {
  grid-column: 2 / 2;
  grid-row: 2 / 2;
}
.select-start-minute {
  grid-column: 3 / 3;
  grid-row: 2 / 2;
}
.label-end {
  grid-column: 1 / 1;
  grid-row: 3 / 3;
}
.select-end-hour {
  grid-column: 2 / 3;
  grid-row: 3 / 3;
}
.select-end-minute {
  grid-column: 3 / 3;
  grid-row: 3 / 3;
}
.label-temperature {
  grid-column: 1 / 1;
  grid-row: 4 / 4;
}
.temperature-input {
  grid-column: 2 / span 3;
  grid-row: 4 / 4;
}
</style>
