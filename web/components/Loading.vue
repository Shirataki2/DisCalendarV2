<template>
  <div style="text-align: center">
    <div :style="styles" class="spinner spinner--stretch">
      <div :style="rectStyles" class="rect rect-1" />
      <div :style="rectStyles" class="rect rect-2" />
      <div :style="rectStyles" class="rect rect-3" />
      <div :style="rectStyles" class="rect rect-4" />
      <div :style="rectStyles" class="rect rect-5" />
    </div>
  </div>
</template>

<script lang="ts">
import { Vue, Component, Prop } from 'nuxt-property-decorator'

@Component({})
class Loading extends Vue {
  @Prop({ type: String, default: '80px' })
  size!: string

  @Prop({ type: String, default: '#8899D4' })
  background!: string

  @Prop({ type: String, default: '0.7s' })
  duration!: string

  get rectStyles () {
    return {
      backgroundColor: this.background,
      animationDuration: this.duration
    }
  }

  get styles () {
    return {
      width: this.size,
      height: this.size
    }
  }
}
export default Loading
</script>

<style lang="scss" scoped>
.spinner {
  display: inline-block;
  text-align: center;
  font-size: 10px;
  * {
    line-height: 0;
    box-sizing: border-box;
  }
}
.rect {
  animation: sk-stretchdelay 1.2s ease-in-out infinite;
}
.spinner > div {
  height: 100%;
  width: 10%;
  display: inline-block;
}
.spinner .rect-1 {
  animation-delay: -1.2s;
}
.spinner .rect-2 {
  animation-delay: -1.1s;
}
.spinner .rect-3 {
  animation-delay: -1s;
}
.spinner .rect-4 {
  animation-delay: -0.9s;
}
.spinner .rect-5 {
  animation-delay: -0.8s;
}
@keyframes sk-stretchdelay {
  0%,
  40%,
  100% {
    transform: scaleY(0.4);
  }
  20% {
    transform: scaleY(1);
  }
}
</style>
