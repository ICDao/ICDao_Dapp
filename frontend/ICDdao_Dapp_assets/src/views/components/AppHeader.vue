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
      <div v-if="user" class="userName">{{ user }}</div>
      <button v-else class="to-white-button login" @click="loginDialog = true">Login</button>
    </div>
    <div v-show="loginDialog" class="login-dialog">
      <div class="close" @click="loginDialog = false"><img :src="require('@/assets/icp/x@2x.png')"></div>
      <div class="title">LOGIN</div>
      <div class="login-button">
        <button @click="login">
          <div>
            <div>Login With</div>
            <div><img :src="require('@/assets/logo@2x.png')"/></div>
          </div>
        </button>
      </div>
      <div class="close-button">
        <button @click="loginDialog = false">CLOSE</button>
      </div>
    </div>
    <div v-show="loginSuccess" class="login-dialog">
      <div class="close" @click="loginSuccess = false"><img :src="require('@/assets/icp/x@2x.png')"></div>
      <div class="title">HELLO</div>
      <div class="message">{{ user }}</div>
      <div class="close-button">
        <button @click="loginSuccess = false">CLOSE</button>
      </div>
    </div>
  </div>
</template>

<script>
import {defineComponent, ref} from "vue";
import {AuthClient} from "@dfinity/auth-client";

export default defineComponent({
  name: "AppHeader",
  setup() {
    const loginSuccess = ref(false)
    let user = ref("")

    const login = async () => {
      loginDialog.value = false;
      const authClient = await AuthClient.create();
      await authClient.login({
        onSuccess: async () => {

          setTimeout(() => {
            user.value = "a2xvk-tkvct-w22yh-myguf-lk3mf-okhta-bc3ro-nngu5-kbb2y-bfqmm-zqe"
            loginSuccess.value = true
            console.log(loginSuccess)
          }, 1000)
        },
      });
      // const canisterId = 'ryjl3-tyaaa-aaaaa-aaaba-cai';
      const identity = await authClient.getIdentity();
      console.log(identity)
      // console.log(byteToString())
      // console.log({canisterId, identity})
      // const actor = Actor.createActor(ICDdao_Dapp_assets, {
      //   agent: new HttpAgent({
      //     identity,
      //   }),
      //   canisterId,
      // });
      // console.log(actor)
    }
    const loginDialog = ref(false);
    return {login, loginDialog, user, loginSuccess}
  }
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

    .userName {
      white-space: nowrap;
      overflow: hidden;
      text-overflow: ellipsis;
      color: wheat;
      width: 300px;
      margin-right: 50px;
      margin-left: auto;
    }

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

.login-dialog {
  .close-button {
    margin-top: 50px;
    width: 157px;
    height: 38px;
    background: #000000;
    margin-left: auto;
    margin-right: auto;

    button {
      text-align: center;
      border: 1px solid #ffffff;
      font-size: 18px;
      font-family: Microsoft YaHei, Microsoft YaHei-Regular;
      font-weight: 400;
      color: #ffffff;
      background-color: black;
      width: 100%;
      height: 100%;
    }

  }

  .close {
    cursor: pointer;
    position: absolute;
    top: 25px;
    right: 25px;
    width: 17px;
    height: 17px;

    img {
      width: 100%;
      height: 100%;
    }
  }

  .message {
    margin-top: 30px;
    font-size: 24px;
    font-family: Microsoft YaHei, Microsoft YaHei-Bold;
    font-weight: 700;
    text-align: center;
    color: #ffffff;
    width: 400px;
    margin-left: auto;
    margin-right: auto;
  }

  .title {
    margin-top: 60px;
    font-size: 24px;
    font-family: Microsoft YaHei, Microsoft YaHei-Bold;
    font-weight: 700;
    text-align: center;
    color: #ffffff;
  }

  .login-button {
    width: 273px;
    height: 63px;
    background: #000000;
    border-radius: 25px;
    margin-left: auto;
    margin-right: auto;
    margin-top: 50px;

    button {
      display: inline-block;
      border-radius: 31px;
      font-size: 20px;
      font-family: Microsoft YaHei, Microsoft YaHei-Regular;
      font-weight: 400;
      text-align: center;
      color: #ffffff;
      width: 100%;
      height: 100%;
      border: 1px solid #f5cd71;
      background-color: black;
      line-height: 50px;
      text-align: center;

      div {
        display: flex;
        flex-direction: row;
        margin-left: auto;
        margin-right: auto;

        img {
          display: block;
          margin-top: auto;
          margin-bottom: auto;
          width: 59px;
          height: 30px;
        }
      }
    }
  }

  margin: auto;
  top: 0;
  bottom: 0;
  left: 0;
  right: 0;
  position: fixed;
  width: 500px;
  height: 400px;
  background: #000000;
  border: 2px solid #ffffff;
  z-index: 1000;
}

</style>
