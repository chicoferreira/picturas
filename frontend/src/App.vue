<script setup lang="ts">
import Navbar from './components/NavBar.vue' 
import { RouterView } from 'vue-router'
</script>

<template>
  <div>
    <Navbar />
    <RouterView :key="$route.fullPath" />
  </div>
</template>