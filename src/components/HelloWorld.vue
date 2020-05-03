<template>
  <div class="hello">
    <div>
      <canvas id="canvas" :width="canvasWidth" :height="canvasHeight">rip</canvas>
    </div>
    <div>
      <button v-on:click="spawn">spawn</button>
    </div>
  </div>
</template>

<script lang="ts">
import { Component, Prop, Vue } from "vue-property-decorator";
import type { SandGame } from "../../pkg/index";

@Component
export default class HelloWorld extends Vue {
  @Prop() private msg!: string;

  //eslint-disable-next-line
  private wasm!: any;
  private sandGame!: SandGame;

  private gameWidth = 256;
  private gameHeight = 256;

  private canvasScale = 3;
  private canvasWidth = this.gameWidth * this.canvasScale;
  private canvasHeight = this.gameHeight * this.canvasScale;

  async mounted() {
    await this.loadWasm();
    this.setupGame();
    requestAnimationFrame(this.renderLoop);
  }

  async loadWasm() {
    const wasm = await import("../../pkg/index");
    this.wasm = wasm;
  }

  setupGame() {
    this.sandGame = this.wasm.SandGame.new(this.gameWidth, this.gameHeight);
    this.sandGame.initialize_webgl();
  }

  renderLoop() {
    this.sandGame.step();
    requestAnimationFrame(this.renderLoop);
  }

  spawn() {
    this.sandGame.spawn(5, 5);
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped lang="scss">
h3 {
  margin: 40px 0 0;
}
ul {
  list-style-type: none;
  padding: 0;
}
li {
  display: inline-block;
  margin: 0 10px;
}
a {
  color: #42b983;
}
</style>
