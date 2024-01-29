<template>
  <div class="settings">
    <table>
      <thead>
        <tr>
          <td>Name</td>
          <td>Value</td>
        </tr>
      </thead>
      <tbody>
        <tr>
          <td>
            Hysteresis :
          </td>
          <td>
            <PrettyNumberInput  class="hysteresis_input" :min=0 :max=2 :step=0.1 v-model="tmp_hysteresis"/>
            <div style="display: none;">{{ tmp_hysteresis }}</div>
          </td>
        </tr>
      </tbody>
    </table>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue'
import axios from 'axios'
import Slider from '@vueform/slider'
import PrettyNumberInput from '@/components/PrettyNumberInput.vue'

export default defineComponent({
  name: 'SettingsView',
  data () {
    return {
      input_holiday_mode_enabled: false,
      tmp_hysteresis: 0,
    }
  },
  mounted() {
    this.tmp_hysteresis = this.$store.state.hysteresis
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
    update_hysteresis: function(value: number) {
      axios
      .put('http://localhost:8080/v1/setting', { key: 'hysteresis', value: value.toString() })
      .then(response => {
        this.$store.commit('update_hysteresis', value)
        console.log(response)
      })
    },
  },
  computed: {
    default_temperature(): String {
      return this.$store.state.default_temperature
    },
    holiday_mode_enabled(): Boolean {
      this.input_holiday_mode_enabled = this.$store.state.holiday_mode_enabled
      return this.$store.state.holiday_mode_enabled
    },
    hysteresis(): Number {
      this.tmp_hysteresis = this.$store.state.hysteresis
      return this.$store.state.hysteresis
    }
  },
  watch: {
    tmp_hysteresis: function(newValue, oldValue) {
      if (newValue != oldValue) {
        this.update_hysteresis(newValue)
      }
    }
  },  components: { Slider, PrettyNumberInput }
})
</script>

<style src="@vueform/slider/themes/default.css"></style>
