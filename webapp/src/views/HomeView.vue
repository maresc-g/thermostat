<template>
  <div class="home">
    <p> Temperature = {{ current_temp }} </p>
      <TimeSlotDay />
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue'
import TimeSlotDay from '@/components/timeslot/TimeSlotDay.vue'

export default defineComponent({
  name: 'HomeView',
  data () {
    return {
    }
  },
  computed: {
    current_temp(): String {
      return this.$store.state.current_temperature
    }
  },
  components: { TimeSlotDay }
})
</script>
