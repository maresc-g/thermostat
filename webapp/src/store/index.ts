import { createStore } from 'vuex'

export default createStore({
  state() {
    return {
      default_temperature: 0.0,
      holiday_mode_enabled: false,
      manual_mode_enabled: false,
      manual_mode_temperature: 0.0,
      is_heating: false,
      hysteresis: 0.0,
      current_temperature: 0.0,
      target_temperature: 0.0,
    }
  },
  getters: {
  },
  mutations: {
    update(state, status) {
      state.default_temperature = status.default_temperature
      state.holiday_mode_enabled = status.holiday_mode_enabled
      state.manual_mode_enabled = status.manual_mode_enabled
      state.manual_mode_temperature = status.manual_mode_temperature
      state.is_heating = status.is_heating
      state.hysteresis = status.hysteresis
      state.current_temperature = status.current_temperature
      state.target_temperature = status.target_temperature
    },
    update_manual_state(state, status) {
      state.manual_mode_enabled = status.manual_mode_enabled
    },
    update_manual_temperature(state, temperature) {
      state.manual_mode_temperature = temperature
    },
    update_hysteresis(state, hysteresis) {
      state.hysteresis = hysteresis
    },
  },
  actions: {
  },
  modules: {
  }
})
