import { createApp } from 'vue'
import App from './App.vue'
import router from './router'
import store from './store'
import Slider from '@vueform/slider'

createApp(App).use(store).use(router).component('Slider', Slider).mount('#app')
