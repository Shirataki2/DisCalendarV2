<template>
  <v-app class="parent">
    <div class="child">
      <p class="status">
        {{ error.statusCode }}
      </p>
      <p class="message">
        一度ログアウトしていただき再度アクセスを試みてください．
      </p>
      <p class="message">
        それでも繰り返しこのエラーがみられる際には
      </p>
      <p class="message">
        サポートサーバーへご連絡ください
      </p>
    </div>
  </v-app>
</template>

<script>
export default {
  layout: 'empty',
  props: {
    error: {
      type: Object,
      default: null
    }
  }
}
</script>

<style scoped>
h1 {
  font-size: 20px;
}
.parent {
  position: relative;
}
.child  {
  position: absolute;
  top: 20%;
  left: 50%;
  transform: translate(-50%, -50%);
  text-align: center;
}
.status {
  font-family: 'Roboto', 'Gill Sans', 'Gill Sans MT', Calibri, 'Trebuchet MS', sans-serif;
  font-weight: 100;
  font-size: 114px;
  letter-spacing: 20px;
  margin-right: -20px;
}
</style>
