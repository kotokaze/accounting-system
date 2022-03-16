<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://v3.vuejs.org/api/sfc-script-setup.html#sfc-script-setup
import { trace, info, error } from 'tauri-plugin-log-api'
import { Store } from 'tauri-plugin-store'
import HelloWorld from './components/HelloWorld.vue'

trace('Loading store')
const store = new Store('store.bin')
store.get<string>('key').then(val => {
  info('value:' + val)
}).catch(() => error('value not found'))

</script>

<template lang="pug">
img(alt="Vue logo", src="./assets/logo.png")
HelloWorld(msg="Hello Vue 3 + TypeScript + Vite")
</template>

<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  margin-top: 60px;
}
</style>
