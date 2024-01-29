<template>
  <app-header />
  <router-view/>
</template>

<script>
import AppHeader from '@/components/AppHeader.vue'
import axios from 'axios'
import store from '@/store'

export default {
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
      .get('http://localhost:8080/v1/status')
      .then(response => (store.commit('update', response.data)))
    }
  },
  components: {
    AppHeader
  }
}

</script>

<style lang="scss">
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
}

nav {

  a {
    font-weight: bold;
    color: #2c3e50;

    &.router-link-exact-active {
      color: #42b983;
    }
  }
}

button {
  background-color: #006eff;
  color: white;
  border: none;
  border-radius: 5px;
  padding: 10px;
}
</style>
