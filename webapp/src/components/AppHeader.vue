<template>
  <header>
    <nav>
      <ul>
        <li><router-link to="/">Home</router-link></li>
        <li><router-link to="/timeslots">Timeslots</router-link></li>
        <li><router-link to="/settings">Settings</router-link></li>
      </ul>
    </nav>
  </header>
</template>

<script lang="ts">
</script>

<style lang="scss">
  header {
    display: flex;
    border-bottom: 1px solid #ccc;

    p {
      margin-left: 1rem;
    }
  }

  nav {
    margin-left: 0;
    padding: 0;

    ul {
      list-style: none;
    }

    ul li {
      display: inline-flex;
      margin-right: 1rem;
    }
  }
</style>
