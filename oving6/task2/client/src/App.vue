<script setup>
import User from './components/User.vue'
</script>

<template>
  <div class="wrapper">
    <h1>Welcome</h1>
    <User/>
  </div>
</template>

<style scoped>
.wrapper {
  margin-top: 10%;
  display: flex;
  flex-direction: column;
  align-content: center;
  align-items: center;
}
</style>
