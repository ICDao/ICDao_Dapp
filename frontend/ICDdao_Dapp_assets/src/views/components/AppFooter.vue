<template>
  <div class="footerBackground"></div>
  <div class="footer">
    <div class="footer">
      <div class="list">
        <div class="title">Company</div>
        <div class="item">Careers</div>
        <div class="item">About us</div>
        <div class="item">Apply for IDO</div>
        <div class="item">Blog</div>
      </div>
      <div class="list">
        <div class="title">Help</div>
        <div class="item">Support</div>
        <div class="item">Terms & Conditions</div>
        <div class="item">Privery Policy</div>
      </div>
    </div>
  </div>
  <div class="trademark">Copyright ©️2021 ****. All rights reserved.</div>
</template>

<script>
import {defineComponent} from "vue";

export default defineComponent({
  name: "AppFooter"
})
</script>

<style lang="scss" scoped>
.footerBackground {
  height: 500px;
  background: linear-gradient(
          0deg, #020202 0%, #271a04 100%);
  width: 300vh;
  margin-left: -30%;
  bottom: 0;
  overflow: auto;
}

.footer {
  display: flex;
  flex-direction: row;
  height: 500px;
  position: absolute;
  bottom: 0;
  .list{
    width: 300px;
    margin-left: 100px;
    .title{
      margin-top: 30px;
      font-size: 23px;
      font-family: Microsoft YaHei, Microsoft YaHei-Bold;
      font-weight: 700;
      text-align: left;
      color: #ffffff;
      line-height: 30px;
    }
    .item{
      cursor: pointer;
      margin-top: 30px;
      font-size: 17px;
      font-family: PingFang SC, PingFang SC-Regular;
      font-weight: 400;
      text-align: left;
      color: #ffffff;
      line-height: 30px;
    }
  }
}
.trademark{
  bottom: 20px;
  text-align: center;
  font-size: 14px;
  font-family: PingFang SC, PingFang SC-Regular;
  font-weight: 400;
  color: #ffffff;
  line-height: 30px;
}
</style>
