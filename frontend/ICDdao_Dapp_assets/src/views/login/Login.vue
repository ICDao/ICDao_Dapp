<template>
  <div class="login">
    <div class="logo">
      <img :src="require('@/assets/logo.png')">
    </div>
    <div class="welcome">Welcome to <br/>Internet Identity</div>
    <div class="text">
      Provide an Identity Anchor to authenticate
      <br/>
      using your Internet Identity.
    </div>
    <div class="anchor">
      <input placeholder="Enter Identity Anchor">
    </div>
    <div class="authenticate">
      <button>Authenticate</button>
    </div>
    <div class="agreement">
      <div>
        <span>New?</span>
        <span class="blue"> Create an Internet Identity Anchor.</span>
      </div>
      <div>
        <span>Already have an anchor</span>
        <span class="blue"> but using a new device?</span>
      </div>
      <div>
        <span>Lost access</span>
        <span class="blue"> and want to recover?</span>
      </div>
    </div>
  </div>

</template>

<script>
import {defineComponent} from "vue";

export default defineComponent({
  name: "Login"
});
</script>

<style lang="scss" scoped>
.login {
  width: 450px;
  background: #000000;
  border: 2px solid #ffffff;
  position: fixed;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);

  .logo {
    width: 50%;
    margin: 0 auto;

    img {
      width: 100%;
      height: auto;
    }
  }

  .welcome {
    margin-left: 50px;
    margin-right: 50px;
    font-size: 24px;
    font-family: Microsoft YaHei, Microsoft YaHei-Bold;
    font-weight: 700;
    text-align: left;
    color: #ffffff;
  }

  .text {
    margin-top: 35px;
    margin-left: 50px;
    margin-right: 50px;
    font-size: 16px;
    font-family: Microsoft YaHei, Microsoft YaHei-Regular;
    font-weight: 400;
    text-align: left;
    color: #ffffff;
  }

  .anchor {
    margin-top: 40px;
    height: 50px;
    margin-left: 50px;
    margin-right: 50px;

    input {
      background: #000000;
      border: 1px solid #ffffff;
      width: calc(100% - 22px);
      height: calc(100% - 2px);

      font-size: 18px;
      font-family: Microsoft YaHei, Microsoft YaHei-Regular;
      font-weight: 400;
      text-align: left;
      color: #ffffff;
      padding-left: 10px;
      padding-right: 10px;

    }
  }

  .authenticate {
    margin-top: 30px;
    height: 50px;
    margin-left: 50px;
    margin-right: 50px;

    button {
      width: 100%;
      height: 100%;
      text-align: center;

      background: #ffffff;
      font-size: 18px;
      font-family: Microsoft YaHei, Microsoft YaHei-Bold;
      font-weight: 700;
      color: #000000;
    }
  }

  .agreement {
    margin: 30px 50px;
    font-size: 16px;
    font-family: Microsoft YaHei, Microsoft YaHei-Regular;
    font-weight: 400;
    text-align: left;
    color: #ffffff;
    line-height: 32px;

    .blue {
      cursor: pointer;
      color: #5079FF;
    }
  }
}
</style>
