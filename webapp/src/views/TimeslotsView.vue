<template>
  <div class="timeslots" style="margin-top: 1rem;">
    <TimeSlotDay />
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue'
import TimeSlotDay from '@/components/timeslot/TimeSlotDay.vue'

export default defineComponent({
  name: 'TimeslotsView',
  data () {
    return {
    }
  },
  methods: {
  },
  components: { TimeSlotDay }
})
</script>

<style src="@vueform/slider/themes/default.css"></style>
