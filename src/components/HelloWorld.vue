<template>
  <div class="hello">
    <canvas id="canvas" width="256" height="256"> </canvas>
    <button v-on:click="spawn">spawn</button>
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
    this.sandGame = this.wasm.SandGame.new(64, 64);
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
