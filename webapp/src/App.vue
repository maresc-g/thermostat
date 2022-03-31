<script setup lang="ts">
</script>

<script lang="ts">
import axios from 'axios'

export default {
  data() {
    return {
      current_temp: '',
      timestamp: ''
    }
  },
  created() {
    setInterval(() => {
      this.getNow();
    }, 5000)
  },
  mounted () {
    this.getNow()
    axios
      .get('http://localhost:8080/v1/temperatures')
      .then(response => (this.current_temp = response["data"]["temperature"]))
  },
  methods: {
    getNow: function() {
      const today = new Date();
      const date = today.getDate().toString().padStart(2, '0') + '/' + (today.getMonth() + 1).toString().padStart(2, '0') + '/' + today.getFullYear();
      const time = today.getHours().toString().padStart(2, '0') + ":" + today.getMinutes().toString().padStart(2, '0');
      const dateTime = date + ' ' + time;
      this.timestamp = dateTime;
    }
  }
}

</script>

<template>
  <header>
    <div>
      Current temperature = {{ current_temp }}
    </div>
    <div>
      Current timestamp = {{ timestamp }}
    </div>
  </header>
</template>

<style>

</style>
