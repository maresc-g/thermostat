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
                      <Multiselect
                        v-model="days_selected"
                        :options="days"
                        mode="tags"
                        :closeOnSelect=false
                        :closeOnDeselect=false
                        class="select-day"
                      />

                      <!-- <select name="days" class="select-day" id="day-select" v-model="day_selected">
                        <option v-for="day in days" :key="day.id">{{ day.name }}</option>
                      </select> -->

                      <label class="label-start">Start Time : </label>
                      <VueDatePicker v-model="start_time" time-picker class="start-time" minutes-increment="5" :start-time="start_time" />

                      <label class="label-end">End Time : </label>
                      <VueDatePicker v-model="end_time" time-picker class="end-time" minutes-increment="5" :start-time="end_time"/>

                      <label for="temperature-input" class="label-temperature">Temperature :</label>
                      <PrettyNumberInput class="temperature-input" :min=14 :max=26 :step=0.5 v-model="temperature_selected" />

                    </div>
                    <div class="modal-footer">
                      <button class="modal-default-button add-button" @click="$emit('addTimeSlot', days_selected, start_time, end_time, temperature_selected)">
                          OK
                      </button>
                      <button class="modal-default-button cancel-button" @click="$emit('cancel')">
                          Cancel
                      </button>
                    </div>
                </div>
            </div>
        </div>
    </transition>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import VueDatePicker from '@vuepic/vue-datepicker';
import '@vuepic/vue-datepicker/dist/main.css'
import PrettyNumberInput from '@/components/PrettyNumberInput.vue'
import Multiselect from '@vueform/multiselect'

defineEmits(['addTimeSlot', 'cancel'])
const days = [
  'Monday', 'Tuesday', 'Wednesday', 'Thursday', 'Friday', 'Saturday', 'Sunday'
]
// const days = [
//   { id: 0, name: 'Monday' },
//   { id: 1, name: 'Tuesday' },
//   { id: 2, name: 'Wednesday' },
//   { id: 3, name: 'Thursday' },
//   { id: 4, name: 'Friday' },
//   { id: 5, name: 'Saturday' },
//   { id: 6, name: 'Sunday' }
// ]
const days_selected = null
const start_time = ref({ hours: 0, minutes: 0 })
const end_time = ref({ hours: 0, minutes: 0 })
const temperature_selected = ref(18.0)
</script>

<style src="@vueform/slider/themes/default.css"></style>
<style src="@vueform/multiselect/themes/default.css"></style>

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
  width: 600px;
  height: 450px;
  margin: 0px auto;
  padding: 10px 20px;
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
  row-gap: 1em;
  column-gap: 1em;
}

.modal-default-button {
  float: right;
  margin-left: 10px;
}

.cancel-button {
  background-color: #d61e1e;
}

.add-button {
  background-color: #009914;
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
  grid-row: 1;
}
.select-day {
  grid-column: 2 / span 2;
  grid-row: 1;
  font-size: 10px;
}
.label-start {
  grid-column: 1 / 1;
  grid-row: 2;
}
.start-time {
  grid-column: 2 / span 2;
  grid-row: 2;
}
.label-end {
  grid-column: 1 / 1;
  grid-row: 3;
}
.end-time {
  grid-column: 2 / span 2;
  grid-row: 3;
}
.label-temperature {
  grid-column: 1 / 1;
  grid-row: 4;
}
.temperature-input {
  grid-column: 2 / span 2;
  grid-row: 4;
}
.multiselect-tag {
  font-size: 10px;
}
</style>
