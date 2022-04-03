<template>
  <header>
    <p> {{ timestamp }}</p>
    <nav>
      <ul>
        <li><router-link to="/">Home</router-link></li>
        <li><router-link to="/settings">Settings</router-link></li>
      </ul>
    </nav>
  </header>
</template>

<script lang="ts">
import { defineComponent } from 'vue'

export default defineComponent({
  data () {
    return {
      timestamp: ''
    }
  },
  created () {
    setInterval(() => {
      this.getNow()
    }, 5000)
  },
  mounted () {
    this.getNow()
  },
  methods: {
    getNow: function () {
      const today = new Date()
      const date = today.getDate().toString().padStart(2, '0') + '/' + (today.getMonth() + 1).toString().padStart(2, '0') + '/' + today.getFullYear()
      const time = today.getHours().toString().padStart(2, '0') + ':' + today.getMinutes().toString().padStart(2, '0')
      const dateTime = date + ' ' + time
      this.timestamp = dateTime
    }
  }
})
</script>

<style lang="scss">
  header {
    display: flex;
    border-bottom: 1px solid #ccc;
    padding: .5rem 1rem;

    p {
      margin-left: 1rem;
    }
  }

  nav {
    margin-left: auto;

    ul {
      list-style: none;
    }

    ul li {
      display: inline-flex;
      margin-left: 1rem;
    }
  }
</style>
