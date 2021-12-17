<template>
  <div :style="{'width':`calc(${width} + 30px)`,'margin-left':'-15px'}" class="rotation-chart">
    <div class="turn-left">
      &lt;
    </div>
    <div class="turn-right">
      >
    </div>
    <div v-for="img in images" :key="img" class="advertisement">
      <img :src="img" alt="advertisement">
    </div>
  </div>
</template>

<script>
import {defineComponent} from "vue";

export default defineComponent({
  name: "RotationChart",
  props: {
    width: {
      type: String
    },
    images: {
      type: Array
    }
  }
});
</script>

<style lang="scss" scoped>
.rotation-chart {
  position: relative;
  display: inline-flex;
  flex-direction: row;

  .turn-left {
    position: absolute;
    width: 33px;
    height: 33px;
    background-color: #00000099;
    top: 50%;
    transform: translateY(-50%);
    left: 0;
    margin-left: 18px;
    margin-right: 18px;
    text-align: center;
    color: white;
    line-height: 30px;
    font-size: 20px;
    border-radius: 50%;
    cursor: pointer;
    border: solid 1px white;
  }

  .turn-right {
    position: absolute;
    width: 33px;
    height: 33px;
    background-color: #00000099;
    right: 0;
    top: 50%;
    transform: translateY(-50%);
    margin-left: 18px;
    margin-right: 18px;
    color: white;
    text-align: center;
    line-height: 30px;
    font-size: 20px;
    border-radius: 50%;
    cursor: pointer;
    border: solid 1px white;
  }

  .advertisement {
    margin-left: 15px;
    margin-right: 15px;
    text-align: center;

    img {
      width: 100%
    }
  }

}

</style>
