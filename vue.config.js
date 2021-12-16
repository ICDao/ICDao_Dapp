const path = require('path')

function initCanisterEnv () {
  let localCanisters,
    prodCanisters
  try {
    localCanisters = require(path.resolve(
      '.dfx',
      'local',
      'canister_ids.json'
    ))
  } catch (error) {
    console.log('No local canister_ids.json found. Continuing production')
  }
  try {
    prodCanisters = require(path.resolve('canister_ids.json'))
  } catch (error) {
    console.log('No production canister_ids.json found. Continuing with local')
  }

  const network =
    process.env.DFX_NETWORK ||
    (process.env.NODE_ENV === 'production' ? 'ic' : 'local')

  const canisterConfig = network === 'local' ? localCanisters : prodCanisters

  return Object.entries(canisterConfig)
    .reduce((prev, current) => {
      const [canisterName, canisterDetails] = current
      prev[canisterName.toUpperCase() + '_CANISTER_ID'] =
        canisterDetails[network]
      return prev
    }, {})
}

process.env = {
  ...process.env,
  ...initCanisterEnv()
}
console.log(process.env)

module.exports = {
  publicPath: '/',
  outputDir: 'dist',
  lintOnSave: process.env.NODE_ENV !== 'production',
  pages: {
    index: {
      entry: 'frontend/ICDdao_Dapp_assets/src/main.js',
      template: 'frontend/ICDdao_Dapp_assets/public/index.html',
      filename: 'index.html',
      title: 'ICDdao_Dapp',
      // 在这个页面中包含的块，默认情况下会包含
      // 提取出来的通用 chunk 和 vendor chunk。
      // chunks: ['chunk-vendors', 'chunk-common', 'index']
    },
    // 当使用只有入口的字符串格式时，
    // 模板会被推导为 `public/subpage.html`
    // 并且如果找不到的话，就回退到 `public/index.html`。
    // 输出文件名会被推导为 `subpage.html`。
    // subpage: 'src/subpage/main.js'
  },
  devServer: {
    proxy: {
      '/api': {
        target: 'http://localhost:8000',
        changeOrigin: true,
        pathRewrite: {
          '^/api': '/api',
        },
      },
    },
    overlay: {
      warnings: true,
      errors: true
    }
  },
  chainWebpack: config => {
    config.resolve.alias
      .set('@', path.resolve('frontend/ICDdao_Dapp_assets/src'))
      .set('$', path.resolve('src'))
  },
}
