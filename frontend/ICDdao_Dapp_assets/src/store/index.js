import { createStore } from 'vuex'
import { AuthClient } from '@dfinity/auth-client'

export default createStore({
  state: {},
  mutations: {},
  actions: {
    login: async () => {
      const authClient = await AuthClient.create()
      console.log('login', authClient)
    }
  },
  modules: {},
})
