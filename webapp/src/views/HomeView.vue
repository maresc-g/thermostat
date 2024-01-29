<template>
  <div class="home">
    <div class="timestamp"> {{ timestamp }}</div>
    <div class="current_temp"> Current Temperature = {{ current_temp }} </div>
    <div class="is_heating"> {{ isHeating }}</div>
    <div class="target_temp"> Target = {{ target_temperature }} (+ {{ hysteresis }})°C</div>
    <div class="switch_state">
      <button @click="switchManualState">
        <span v-if="manualState">Stop manual mode</span>
        <span v-else>Start manual mode</span>
      </button>
    </div>
    <div class="manual_temperature">
      <div class="manual_temperature_div">
        <div class="manual_temperature_label">Manual temperature : </div>
        <PrettyNumberInput  class="manual_temperature_input" :min=14 :max=30 :step=0.5 v-model="tmp_manual_mode_temperature"/>
        <div style="display:none">{{ manual_mode_temperature }}</div>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref } from 'vue'
import TimeSlotDay from '@/components/timeslot/TimeSlotDay.vue'
import PrettyNumberInput from '@/components/PrettyNumberInput.vue'
import axios from 'axios'

export default defineComponent({
  name: 'HomeView',
  data () {
    return {
      timestamp: '',
      tmp_manual_mode_temperature: 0,
      value: null,
        options: [
          'Batman',
          'Robin',
          'Joker',
        ]
    }
  },
  created () {
    setInterval(() => {
      this.setup()
    }, 5000)
  },
  mounted () {
    this.setup()
  },
  methods: {
    switchManualState () {
      axios
      .post('http://localhost:8080/v1/manual/' + (!this.manualState ? 'true' : 'false'))
      .then(response => {
        console.log(response)
        this.manualState = response.data.value === 'true'
      })
    },
    setup: function() {
      this.getNow()
    },
    getNow: function () {
      const today = new Date()
      const date = today.getDate().toString().padStart(2, '0') + '/' + (today.getMonth() + 1).toString().padStart(2, '0') + '/' + today.getFullYear()
      const time = today.getHours().toString().padStart(2, '0') + ':' + today.getMinutes().toString().padStart(2, '0')
      const dateTime = date + ' ' + time
      this.timestamp = dateTime
    },
    update_manual_temperature: function(value: number) {
      axios
      .put('http://localhost:8080/v1/setting', { key: 'manual_mode_temperature', value: value.toString() })
      .then(response => {
        this.$store.commit('update_manual_temperature', value)
        console.log(response)
      })
    },
  },
  computed: {
    current_temp(): String {
      return this.$store.state.current_temperature
    },
    manualState(): Boolean {
      return this.$store.state.manual_mode_enabled
    },
    isHeating(): String {
      if (this.$store.state.is_heating) {
        return "HEATING ON"
      }
      else {
        return "HEATING OFF"
      }
    },
    target_temperature(): Number {
      return this.$store.state.target_temperature
    },
    hysteresis(): Number {
      return this.$store.state.hysteresis
    },
    manual_mode_temperature(): Number {
      this.tmp_manual_mode_temperature = this.$store.state.manual_mode_temperature
      return this.$store.state.manual_mode_temperature
    },
  },
  watch: {
    tmp_manual_mode_temperature: function(value) {
      this.update_manual_temperature(value)
    }
  },
  components: { TimeSlotDay, PrettyNumberInput }
})
</script>

<style>
html {
  font-size: x-large;
}
button {
  font-size: large;
}

.home {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  grid-gap: 10px;
  grid-auto-rows: minmax(100px, auto);
}
.timestamp {
  grid-column: 1;
  grid-row: 1;
  align-self: center;
}
.current_temp {
  grid-column: 2;
  grid-row: 1;
  align-self: center;
}
.target_temp {
  grid-column: 3;
  grid-row: 1;
  align-self: center;
}
.is_heating {
  grid-column: 1;
  grid-row: 2;
  align-self: center;
}
.switch_state {
  grid-column: 2;
  grid-row: 2;
  align-self: center;
}
.manual_temperature {
  grid-column: 3;
  grid-row: 2;
  align-self: center;
}

.manual_temperature_div {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  grid-gap: 10px;
}
.manual_temperature_label {
  grid-column: 1;
  grid-row: 1;
  align-self: center;
}
.manual_temperature_input {
  grid-column: 2;
  grid-row: 1;
  align-self: center;
}
</style>
