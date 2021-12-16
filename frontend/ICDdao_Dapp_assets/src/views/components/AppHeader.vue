<template>
  <div class="app-header">
    <div class="title-bar">
      <router-link active-class="active" class="page" to="/">HomePage</router-link>
      <span class="page">Projects</span>
      <span class="page">NFT Market</span>
      <span class="page">Vote</span>
      <span class="page">Mine</span>
    </div>
    <div class="header-right">
      <router-link to="/login">
        <button class="white-button login">login</button>
      </router-link>
    </div>
  </div>
</template>

<script>
import {defineComponent} from "vue";

export default defineComponent({
  name: "AppHeader"
})
</script>

<style lang="scss" scoped>
.app-header {
  margin-top: 40px;
  display: flex;
  flex-direction: row;

  .title-bar {
    line-height: 23px;
    display: flex;
    flex-direction: row;

    .page {
      font-size: 18px;
      margin-left: 30px;
      margin-right: 30px;
      cursor: pointer;
    }

    .active {
      font-weight: 700;
      text-decoration-line: unset;
      font-size: 20px;
      color: black;
    }
  }

  .header-right {
    flex: 1;
    text-align: right;

    .login {
      width: 140px;
      height: 44px;
      line-height: 23px;
      border-radius: 22px;
      color: black;
      font-size: 16px;
      background-color: #fff;
      text-align: center;
      border: 1px solid rgba(187, 187, 187, 100);
      cursor: pointer;
    }
  }
}


</style>
