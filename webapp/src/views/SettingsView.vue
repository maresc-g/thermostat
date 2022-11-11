<template>
  <div class="settings">
    <h1>This is the settings page</h1>
    <p>Default temperature = {{ default_temperature }} </p>
      <div>
        <input type="checkbox" v-model="input_holiday_mode_enabled" id="holiday_mode" name="holiday_mode" @change="onInput" :value="holiday_mode_enabled" >
        <label for="holiday_mode">Holiday Mode</label>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue'
import axios from 'axios'

export default defineComponent({
  name: 'SettingsView',
  data () {
    return {
      default_temperature: '',
      holiday_mode_enabled: false,
      input_holiday_mode_enabled: false
    }
  },
  mounted () {
    this.getSettings()
  },
  methods: {
    getSettings: function () {
      axios
        .get('http://thermostat:8080/v1/setting')
        .then(response => {
          const settings = response.data.settings
          this.default_temperature = settings.default_temperature
          this.holiday_mode_enabled = settings.holiday_mode_enabled === 'True'
        })
    },
    onInput () {
      console.log(this.input_holiday_mode_enabled)
      axios
        .put('http://thermostat:8080/v1/setting', { key: 'holiday_mode_enabled', value: this.input_holiday_mode_enabled.toString() })
        .then(response => {
          console.log(response)
        })
    }
  }
})
</script>
