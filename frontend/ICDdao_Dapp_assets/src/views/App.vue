<template>
  <RouterView></RouterView>
</template>
<script>
import {defineComponent, onMounted} from "vue";
import {useStore} from "vuex";

export default defineComponent({
  name: "App",
  setup() {
    const vuex = useStore()
    onMounted(() => {
      vuex.dispatch('login')
    })
  }
});
</script>
