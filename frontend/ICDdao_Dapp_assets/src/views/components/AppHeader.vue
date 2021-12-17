<template>
  <div class="app-header">
    <div class="logo">
      <router-link to="/">
        <img :src="require('@/assets/icp/logo@2x.png')">
      </router-link>
    </div>
    <div class="title-bar">
      <router-link active-class="active" class="page" to="/">HomePage</router-link>
      <span class="page">Projects</span>
      <span class="page">NFT Market</span>
      <span class="page">Vote</span>
      <span class="page">Mine</span>
    </div>
    <div class="header-right">
      <router-link to="/login">
        <button class="to-white-button login">Login</button>
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
@keyframes page {
  from {
    top: 0;
  }
  to {
    top: 5px;
  }
}

.app-header {
  margin-top: 40px;
  display: flex;
  flex-direction: row;

  .title-bar {
    margin-top: 5px;
    color: white;
    line-height: 23px;
    display: flex;
    flex-direction: row;

    .page {
      font-size: 18px;
      margin-left: 30px;
      margin-right: 30px;
      cursor: pointer;

      &:hover {
        position: relative;
        animation: page 200ms forwards;
      }
    }

    .active {
      font-weight: 700;
      text-decoration-line: unset;
      font-size: 20px;
      color: white;
    }
  }

  .header-right {
    flex: 1;
    text-align: right;

    .login {
      width: 78px;
      height: 31px;
      background: linear-gradient(0deg, #ffe08c 0%, #d99927 100%);
      border-radius: 16px;

      font-size: 16px;
      font-family: Microsoft YaHei, Microsoft YaHei-Regular;
      font-weight: 400;
      text-align: center;
      color: #ffffff;
    }
  }

  .logo {
    cursor: pointer;
    margin-left: 213px;
    height: 36px;
    width: 114px;

    img {
      width: 100%;
      height: 100%;
    }
  }
}


</style>
