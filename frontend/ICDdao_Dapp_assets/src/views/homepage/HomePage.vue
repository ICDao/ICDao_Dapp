<template>
  <div class="background">
    <img :src="require('@/assets/icp/banner@2x.png')">
  </div>
  <app-header></app-header>
  <div class="about">
    <div class="left">
      <div class="title">
        <div>Get early access</div>
        <div>to the ideas of tomorrow</div>
      </div>
      <div class="describe">
        <div>Highly-vetted ideas and teams you can trust.Supported by industry-leading creators and funds.</div>
      </div>
      <div class="see-all-project">
        <button class="to-white-button">See All Project</button>
      </div>
    </div>
  </div>
  <rotation-chart :images="advertisements" class="rotation-chart" width="100%"></rotation-chart>
  <div class="vote">
    <div class="title">Vote</div>
    <div class="details">
      <div class="card">
        <div class="suc">
          <img :src="require('@/assets/icp/suc@2x.png')" alt="suc">
        </div>
        <div class="text-popular">
          Popular Votes for you
        </div>
        <div class="text-community">
          Functional management community,
          <br/>
          select high-quality projects together
        </div>
        <div class="all-votes">
          <button class="to-white-button">All Votes ➡</button>
        </div>
      </div>
      <div class="vote-list">
        <div v-for="item in 3" :key="item" class="vote-item">
          <div class="thumbnail">
            <img :src="require('@/assets/advertisement/1@2x.png')"/>
          </div>
          <div class="vote-details">
            <div class="name">Polygon-Grants Hackathon Round...</div>
            <div class="date-tag">
              <div class="date">
                <div class="text">End after</div>
                <div class="time">0d 03h 24m 26s</div>
              </div>
              <div class="tag">WEB3</div>
            </div>
            <div class="thumbs">
              <div class="up-img">
                <img :src="require('@/assets/icp/zan@2x.png')">
              </div>
              <div class="up">thumbs</div>
              <div class="down-img">
                <img :src="require('@/assets/icp/cai@2x.png')">
              </div>
              <div class="down">thumbs</div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
  <div class="projects">
    <div class="title">Upcoming Projects</div>
    <div class="projects-list">
      <div v-for="item in 3" :key="item" class="projects-item">
        <div class="card-header">
          <div class="user">
            <img :src="require('@/assets/user/5.jpeg')">
          </div>
          <div class="name">Close In</div>
          <div class="time">8d 09h 15m 24s</div>
        </div>
        <div :style="{'background':backgroundColor[item-1]}" class="card-img">
          <img :src="require('@/assets/advertisement/5@2x.png')">
        </div>
        <div class="card-describe">
          <div class="token">
            <div class="name">Project Name</div>
            <div class="type">
              <span>Token</span>
              <span class="tag">WEB3</span>
            </div>
          </div>
          <div class="token-img">
            <img :src="require('@/assets/icp/ETH@2x.png')">
          </div>
        </div>
        <div class="card-details">
          <div class="detail">
            <span class="key">Total Raise</span>
            <span class="value">$500,000</span>
          </div>
          <div class="detail">
            <span class="key">Token</span>
            <span class="value">$500 Max</span>
          </div>
        </div>
      </div>
    </div>
  </div>
  <div class="learn">
    <div class="title">Learn about</div>
    <div class="learn-list">
      <div class="learn-item" v-for="item in 3" :key="item">
        <div class="top-img">
          <img :src="require('@/assets/advertisement/7@2x.png')">
        </div>
        <div class="question">
          How to Participate in a perfect IDO?
        </div>
        <div class="answer">
          A good place to start is:what is ****? (We'll give you the brief verson).
          *** is a platform that connects young projects with early...
        </div>
      </div>
    </div>
  </div>
  <div class="submit-project">
    <button class="to-white-button">&#8679; Submit Project</button>
  </div>
  <app-footer></app-footer>
</template>

<script>
import {defineComponent, onMounted, ref} from "vue";
import AppHeader from "../components/AppHeader";
import RotationChart from "@/views/homepage/RotationChart";
import AppFooter from "@/views/components/AppFooter";

export default defineComponent({
  name: "HomePage",
  components: {AppFooter, RotationChart, AppHeader},
  setup() {
    const backgroundColor = ref(["#bdcdff", "#dfd5ff", "#9cd8f8"])
    const advertisements = ref([]);
    onMounted(() => {
      for (let num of [1, 2, 3, 4]) {
        advertisements.value.push(require(`@/assets/advertisement/${num}@2x.png`))
      }
    })
    return {advertisements, backgroundColor};
  }
});
</script>

<style lang="scss" scoped>
.submit-project{
  position: absolute;
  bottom: 500px;
  right: 0;
  button{
    cursor: pointer;
    width: 227px;
    height: 58px;
    background: linear-gradient(0deg,#ffe08c 0%, #d99927 100%);
    border-radius: 10px;

    font-size: 20px;
    font-family: Microsoft YaHei, Microsoft YaHei-Bold;
    font-weight: 700;
    text-align: center;
    color: #ffffff;
  }
}
.about {
  display: flex;
  flex-direction: column;

  .title {
    margin-top: 100px;
    color: rgba(242, 128, 72, 100);
    font-size: 54px;
    font-family: Microsoft YaHei, Microsoft YaHei-Bold;
    font-weight: 700;
    text-align: center;
    color: #ffffff;
    text-shadow: 0 0 8px 0 #00000063;
  }

  .describe {
    font-size: 20px;
    font-family: Microsoft YaHei, Microsoft YaHei-Regular;
    font-weight: 400;
    text-align: center;
    color: #ffffff;
    line-height: 44px;
    text-shadow: 0 0 8px 0 #000000;

    margin-top: 60px;
  }

  .see-all-project {
    margin-top: 60px;
    height: 60px;
    text-align: center;

    button {
      width: 258px;
      height: 64px;
      background: #000000BF;
      border: 2px solid #ffffff;

      font-size: 20px;
      font-family: Microsoft YaHei, Microsoft YaHei-Bold;
      font-weight: 700;
      text-align: center;
      color: #ffffff;

      cursor: pointer;
    }
  }
}

.background {
  z-index: -1;
  top: 50px;
  position: absolute;
  width: 490px;
  height: 735px;

  margin-left: 50%;
  transform: translateX(-50%);

  img {
    width: 100%;
    height: 100%;
  }
}

.rotation-chart {
  margin-top: 200px;
}

.vote {
  margin-top: 100px;

  .title {
    font-size: 40px;
    font-family: Microsoft YaHei, Microsoft YaHei-Bold;
    font-weight: 700;
    text-align: left;
    color: #ffffff;
  }

  .details {
    min-height: 550px;
    display: flex;
    flex-direction: row;
    margin-top: 70px;

    .card {
      width: 43%;
      text-align: center;
      background: #191919;
      margin-right: 80px;

      .suc {
        margin-top: 71px;
        width: 167px;
        height: 142px;
        margin-left: auto;
        margin-right: auto;

        img {
          width: 100%;
          height: 100%;
        }
      }

      .text-popular {
        margin-top: 55px;
        font-size: 24px;
        font-family: FZLanTingHeiS-H-GB, FZLanTingHeiS-H-GB-Regular;
        font-weight: 400;
        color: #ffffff;
      }

      .text-community {
        margin-top: 36px;
        font-size: 20px;
        font-family: PingFang SC, PingFang SC-Regular;
        font-weight: 400;
        color: #ffffff;
        line-height: 28px;
      }

      .all-votes {
        margin-top: 60px;

        button {
          font-size: 21px;
          font-family: Microsoft YaHei, Microsoft YaHei-Bold;
          font-weight: 700;
          color: #ffffff;
          cursor: pointer;
          width: 205px;
          height: 49px;
          background: linear-gradient(0deg, #ffe08c 0%, #d99927 100%);
        }
      }

    }

    .vote-list {
      flex: 1;

      .vote-item {
        margin-bottom: 40px;
        height: 162px;
        background: #232323;
        display: flex;
        flex-direction: row;

        &:nth-last-child(1) {
          margin-bottom: 0;
        }

        .thumbnail {
          width: 170px;
          height: 120px;
          margin: auto 20px;

          img {
            width: 100%;
            height: 100%;
          }
        }

        .vote-details {
          flex: 1;
          position: relative;
          height: 120px;
          margin: auto 20px;

          .name {
            font-size: 24px;
            font-family: PingFang SC, PingFang SC-Semibold;
            font-weight: 600;
            text-align: left;
            color: #ffffff;
          }

          .date-tag {
            display: flex;
            flex-direction: row;
            margin-top: 15px;

            .date {
              display: flex;
              flex-direction: row;

              .text {
                font-size: 20px;
                font-family: PingFang SC, PingFang SC-Semibold;
                font-weight: 600;
                text-align: left;
                color: #ffffff;
              }

              .time {
                margin-left: 20px;
                font-size: 18px;
                font-family: Microsoft YaHei, Microsoft YaHei-Regular;
                font-weight: 400;
                text-align: left;
                color: #fcd380;
              }
            }

            .tag {

              padding: 0 5px;
              margin-left: 30px;
              width: 51px;
              height: 24px;
              line-height: 24px;
              background: #fcd38099;
              border-radius: 12px;
              font-size: 14px;
              font-family: Microsoft YaHei, Microsoft YaHei-Regular;
              font-weight: 400;
              color: #fcd380;
              text-align: center;
            }
          }

          .thumbs {
            margin-top: 15px;
            display: flex;
            flex-direction: row;

            .up-img, .down-img {
              cursor: pointer;
              width: 23px;
              height: 23px;

              img {
                width: 100%;
                height: 100%;
              }
            }

            .down-img {
              margin-left: 50px;
            }

            .up, .down {
              cursor: pointer;

              line-height: 23px;
              margin-left: 15px;
              font-size: 20px;
              font-family: PingFang SC, PingFang SC-Semibold;
              font-weight: 600;
              text-align: left;
              color: #ffffff;
            }
          }
        }
      }
    }
  }
}

.learn {
  margin-top: 100px;
  margin-bottom: 100px;

  .title {
    font-size: 40px;
    font-family: Microsoft YaHei, Microsoft YaHei-Bold;
    font-weight: 700;
    text-align: left;
    color: #ffffff;
  }

  .learn-list {
    margin-top: 70px;
    transform: translateX(-1%);
    width: 102%;
    display: flex;
    flex-direction: row;

    .learn-item {
      margin-left: 1%;
      margin-right: 1%;
      width: 32%;
      background: #ffffff;
      border-radius: 10px;
      height: 0;
      padding-bottom: 40%;

      .question {
        margin-left: 30px;
        margin-right: 30px;
        margin-top: 30px;
        font-size: 21px;
        font-family: Microsoft YaHei, Microsoft YaHei-Bold;
        font-weight: 700;
        text-align: left;
        color: #333333;
      }

      .answer {
        margin-left: 30px;
        margin-right: 30px;
        margin-top: 30px;
        font-size: 18px;
        font-family: PingFang SC, PingFang SC-Regular;
        font-weight: 400;
        text-align: left;
        color: #333333;
        line-height: 28px;
      }

      .top-img {
        width: 100%;

        img {
          width: 100%;
        }
      }
    }
  }
}

.projects {
  margin-top: 100px;

  .title {
    font-size: 40px;
    font-family: Microsoft YaHei, Microsoft YaHei-Bold;
    font-weight: 700;
    text-align: left;
    color: #ffffff;
  }

  .projects-list {
    //margin-left: 1%;
    margin-top: 70px;
    transform: translateX(-1%);
    width: 102%;
    display: flex;
    flex-direction: row;

    .projects-item {
      margin-left: 1%;
      margin-right: 1%;
      width: 32%;
      background: #ffffff;
      border-radius: 10px;

      .card-header {
        height: 70px;
        display: flex;
        flex-direction: row;
        line-height: 70px;

        .user {
          margin-left: 50px;
          margin-top: auto;
          margin-bottom: auto;
          height: 40px;
          width: 40px;

          img {
            height: 100%;
            width: 100%;
          }
        }

        .name {
          margin-left: 30px;
          font-size: 20px;
          font-family: Microsoft YaHei, Microsoft YaHei-Bold;
          font-weight: 700;
          color: #333333;
        }

        .time {
          flex: 1;
          text-align: right;
          margin-right: 50px;
          font-size: 16px;
          font-family: Microsoft YaHei, Microsoft YaHei-Regular;
          font-weight: 400;
          color: #333333;
        }
      }

      .card-img {
        margin: 0 auto;
        padding: 20px 10%;

        img {
          box-shadow: 0px 2px 10px 0px rgba(21, 21, 21, 0.70);
          width: 100%;
        }
      }

      .card-describe {
        width: 80%;
        margin-top: 30px;
        margin-left: 10%;
        margin-right: 10%;
        display: flex;
        flex-direction: row;

        .token {
          flex: 1;

          .name {
            font-size: 20px;
            font-family: Microsoft YaHei, Microsoft YaHei-Bold;
            font-weight: 700;
            text-align: left;
            color: #333333;
          }

          .type {
            font-size: 17px;
            font-family: Microsoft YaHei, Microsoft YaHei-Regular;
            font-weight: 400;
            color: #333333;

            .tag {
              margin-left: 30px;
              padding-left: 10px;
              padding-right: 10px;
              font-size: 14px;
              font-family: Microsoft YaHei, Microsoft YaHei-Regular;
              font-weight: 400;
              color: #f0b134;

              height: 23px;
              opacity: 0.34;
              background: #fcd380;
              border-radius: 12px;
            }
          }
        }

        .token-img {
          height: 50px;
          width: 50px;

          img {
            height: 100%;
            width: 100%;
          }
        }
      }

      .card-details {
        width: 80%;
        margin: 30px 10%;

        display: flex;
        flex-direction: column;

        .detail {
          font-size: 17px;
          font-family: Microsoft YaHei, Microsoft YaHei-Regular;
          font-weight: 400;
          color: #333333;

          .key {
            float: left;
          }

          .value {
            float: right;
          }
        }
      }
    }
  }
}
</style>
