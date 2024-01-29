<template>
    <div class="main">
        <button class="minus" @click="minus">-</button>
        <input ref="input" class="number" type="number" :min="min" :max="max" :step="step" v-model="model"/>
        <button class="plus" @click="plus">+</button>
    </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
const model = defineModel({type: Number, required: true})
const props = defineProps({
    min: {
        type: Number,
        required: true
    },
    max: {
        type: Number,
        required: true
    },
    step: {
        type: Number,
        required: true
    },
})
const input = ref()
onMounted(()=>{
    input.value.focus()
    input.value.value = model.value
})

function minus() {
    if (model != null && model.value - props.step >= props.min) {
        model.value = Number.parseFloat((model.value - props.step).toFixed(2))
    }
}
function plus() {
    if (model != null && model.value + props.step <= props.max) {
        model.value = Number.parseFloat((model.value + props.step).toFixed(2))
    }
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
    column-gap: 5px;
    grid-template-columns: auto 1fr auto
}

.minus {
    grid-column: 1;
    background-color: #d61e1e;
    width: 50px;
}

.number {
    grid-column: 2;
}

.plus {
    grid-column: 3;
    background-color: #009914;
    width: 50px;
}
</style>
