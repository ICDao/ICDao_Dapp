import { createApp } from 'vue'
import App from './views/App'
import VueRouter from './router'
import VueX from './store'
import "./style/main.css"
createApp(App)
  .use(VueRouter)
  .use(VueX)
  .mount('#app')
