<template>
  <div class="settings">
    <h1>This is the settings page</h1>
    <p>Default temperature = {{ default_temperature }} </p>
    <div>
      <input type="checkbox" v-model="input_holiday_mode_enabled" id="holiday_mode" name="holiday_mode" @change="onInput" :value="holiday_mode_enabled" >
      <label for="holiday_mode">Holiday Mode</label>
      <Slider
      v-model="manual_mode_temperature"
      :format="format"
      :min="10"
      :max="30"
      :step="0.5"
      @change="update_manual_temperature"
    />
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue'
import axios from 'axios'
import Slider from '@vueform/slider'

export default defineComponent({
  name: 'SettingsView',
  data () {
    return {
      input_holiday_mode_enabled: false,
      value: 18,
      format: function (value: number) {
        return `${value}°C`
      }
    }
  },
  methods: {
    onInput () {
      console.log(this.input_holiday_mode_enabled)
      axios
        .put('http://localhost:8080/v1/setting', { key: 'holiday_mode_enabled', value: this.input_holiday_mode_enabled.toString() })
        .then(response => {
          console.log(response)
        })
    },
    update_manual_temperature: function(value: number) {
      axios
        .put('http://localhost:8080/v1/setting', { key: 'manual_mode_temperature', value: value.toString() })
        .then(response => {
          this.$store.commit('update_manual_temperature', value)
          console.log(response)
        })
    }
  },
  computed: {
    default_temperature(): String {
      return this.$store.state.default_temperature
    },
    holiday_mode_enabled(): Boolean {
      this.input_holiday_mode_enabled = this.$store.state.holiday_mode_enabled
      return this.$store.state.holiday_mode_enabled
    },
    manual_mode_temperature(): Number {
      return this.$store.state.manual_mode_temperature
    },
  },
  components: { Slider }
})
</script>

<style src="@vueform/slider/themes/default.css"></style>
