/*import './assets/main.css'    */
import "@/assets/main.css"

import { createApp } from 'vue'
import { createPinia } from 'pinia'

import App from './App.vue'
/*import { PiniaPluginPersistedState } from 'pinia-plugin-persistedstate' */

import router from '@/router'

const app = createApp(App)

/*
const pinia = createPinia()
pinia.use(PiniaPluginPersistedState*/

app.use(createPinia())
app.use(router)

app.mount('#app')
