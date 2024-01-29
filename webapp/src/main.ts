import { createApp } from 'vue'
import App from './App.vue'
import router from './router'
import store from './store'
import Slider from '@vueform/slider'
import VueDatePicker from '@vuepic/vue-datepicker';
import '@vuepic/vue-datepicker/dist/main.css'
import Multiselect from '@vueform/multiselect'

createApp(App)
    .use(store)
    .use(router)
    .component('Slider', Slider)
    .component('VueDatePicker', VueDatePicker)
    .component('MultiSelect', Multiselect)
    .mount('#app')
