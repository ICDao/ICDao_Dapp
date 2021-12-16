<template>
  <app-header></app-header>
  <div class="about">
    <div class="left">
      <div class="title">
        <div>Get early access</div>
        <div>to the ideas of tomorrow</div>
      </div>
      <div class="describe">
        <div>Highly-vetted ideas and teams you can trust.Supported by</div>
        <div>industry-leading creators and funds.</div>
      </div>
      <div class="see-all-project">
        <button class="to-white-button">See All Project</button>
      </div>
    </div>
    <div class="right">
      <img alt="earth" class="earth" :src="require('@/assets/logo.png')">
    </div>
  </div>

</template>

<script>
import {defineComponent} from "vue";
import AppHeader from "../components/AppHeader";

export default defineComponent({
  name: "HomePage",
  components: {AppHeader}
});
</script>

<style lang="scss" scoped>
.about {
  display: flex;
  flex-direction: row;

  .left {
    display: flex;
    flex-direction: column;
    .title{
      margin-top: 100px;
      width: 600px;
      height: 100px;
      color: rgba(242, 128, 72, 100);
      font-size: 36px;
    }
    .describe{
      margin-top: 60px;
      width: 500px;
      height: 60px;
      color: rgba(34, 34, 34, 100);
      font-size: 18px;
    }
    .see-all-project{
      margin-top: 60px;
      height: 60px;
      button{
        width: 277px;
        height: 100%;
        line-height: 26px;
        border-radius: 30px;
        border:unset;
        color: black;
        border: 1px #767676 solid;
        background-color: rgba(242, 128, 72, 100);
        //color: rgba(248, 248, 248, 100);
        font-size: 18px;
        text-align: center;
        cursor: pointer;
      }
    }
  }
  .right{
    margin: auto;
    padding-top: 50px;
    padding-left: 50px;
    img{
      width: 256px;
      height: 256px;
    }
  }
}
</style>
