<template>
  <div class="home">
    <p> Temperature = {{ current_temp }} </p>
    <TimeSlotDay />
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue'
import axios from 'axios'
import TimeSlotDay from '@/components/timeslot/TimeSlotDay.vue'

export default defineComponent({
  name: 'HomeView',
  data () {
    return {
      current_temp: ''
    }
  },
  created () {
    setInterval(() => {
      this.updateTemp()
    }, 5000)
  },
  mounted () {
    this.updateTemp()
  },
  methods: {
    updateTemp: function () {
      axios
        .get('http://localhost:8080/v1/temperatures')
        .then(response => (this.current_temp = response.data.temperature))
    }
  },
  components: { TimeSlotDay }
})
</script>