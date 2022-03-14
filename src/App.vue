<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://v3.vuejs.org/api/sfc-script-setup.html#sfc-script-setup
import { trace, info, error, attachConsole } from 'tauri-plugin-log-api'
import HelloWorld from './components/HelloWorld.vue'

// with LogTarget::Webview enabled this function will print logs to the browser console
const detach = await attachConsole()

trace('Trace')
info('Info')
error('Error')

// detach the browser console from the log stream
detach()
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
