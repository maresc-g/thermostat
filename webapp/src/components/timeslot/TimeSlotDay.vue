<template>
    <div class="timeslot_day">
        <p> {{ day }} </p>
        <div class="wrapper">
          <div class="row" :class="time.class" v-for="time in times" :key="time.id">
            <div class="time">{{ time.value }}</div>
          </div>
        </div>
    </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue'

export default defineComponent({
  props: {
    day: String
  },
  data () {
    return {
      times: [] as { id: number, value: string, class: string }[]
    }
  },
  created () {
    for (let i = 0; i < 24; i++) {
      const j = { id: i, value: i.toString().padStart(2, '0') + ':00', class: 'hour' }
      const j2 = { id: i, value: i.toString().padStart(2, '0') + ':30', class: 'half_hour' }
      this.times.push(j)
      this.times.push(j2)
    }
  }
  // },
  // methods: {
  //   addTimeSlot (event) {
  //     console.log('Hello ' + event.target)
  //   }
  // }
})
</script>

<style lang="scss">
  .wrapper {
    display: grid;
    grid-template-rows: repeat(48, 50);
  }
  .row {
    text-align: left;
  }
  .hour {
    border: grey dashed;
    border-width: 0 0 1px 0;
  }
  .half_hour {
    border: grey solid;
    border-width: 0 0 1px 0;
  }
</style>
