<template>
    <div class="main">
        <button class="minus" @click="minus">-</button>
        <input ref="input" class="number" type="number" :min="min" v-model="value" :max="max" :step="step"/>
        <button class="plus" @click="plus">+</button>
    </div>
</template>

<script setup lang="ts">
import { ref,onMounted } from 'vue'
const props = defineProps(['min', 'max', 'step'])
const value = defineModel()
const input = ref()
onMounted(()=>{
    input.value.focus()
})

function minus() {
    input.value.stepDown()
}
function plus() {
    input.value.stepUp()
}
</script>


<style lang="scss">

input[type="number"] {
  -webkit-appearance: textfield;
  -moz-appearance: textfield;
  appearance: textfield;
}

input[type=number]::-webkit-inner-spin-button,
input[type=number]::-webkit-outer-spin-button {
  -webkit-appearance: none;
}

.main {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
}

.minus {
    grid-column: 1;
}

.number {
    grid-column: 2;
}

.plus {
    grid-column: 3;
}
</style>
