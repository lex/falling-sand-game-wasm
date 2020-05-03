<template>
  <div class="hello">
    <div>
      <canvas
        id="canvas"
        :width="canvasWidth"
        :height="canvasHeight"
        v-on:mousemove="onMouseMove"
        v-on:mousedown="onMouseDown"
        v-on:mouseup="onMouseUp"
        >rip</canvas
      >
    </div>
    <div>
      <p>{{ particleTypeAsString(particleType) }}</p>
    </div>
    <div>
      <button
        v-for="particleType in particleTypes"
        :key="particleType"
        v-on:click="selectType(particleType)"
      >
        {{ particleTypeAsString(particleType) }}
      </button>
    </div>
  </div>
</template>

<script lang="ts">
import { Component, Prop, Vue } from "vue-property-decorator";
import type { SandGame } from "../../pkg/index";

enum ParticleType {
    Empty = 0,
    Wall = 1,
    Sand = 2,
    Water = 3,
    Plant = 4,
}

@Component
export default class Game extends Vue {
  //eslint-disable-next-line
  private wasm!: any;
  private sandGame!: SandGame;

  private gameWidth = 256;
  private gameHeight = 256;

  private canvasScale = 3;
  private canvasWidth = this.gameWidth * this.canvasScale;
  private canvasHeight = this.gameHeight * this.canvasScale;

  private mouseX = 0;
  private mouseY = 0;
  private drawing = false;
  private particleType = ParticleType.Sand;

  async mounted() {
    await this.loadWasm();
    this.setupGame();
    requestAnimationFrame(this.renderLoop);
  }

  private async loadWasm() {
    const wasm = await import("../../pkg/index");
    this.wasm = wasm;
  }

  private onMouseMove(event: MouseEvent) {
    this.mouseX = Math.floor(event.offsetX / this.canvasScale);
    this.mouseY = Math.floor(event.offsetY / this.canvasScale);
  }

  private onMouseDown() {
    this.drawing = true;
  }

  private onMouseUp() {
    this.drawing = false;
  }

  private setupGame() {
    this.sandGame = this.wasm.SandGame.new(this.gameWidth, this.gameHeight);
    this.sandGame.initialize_webgl();
  }

  private draw() {
    const r = 10;
    const x = this.mouseX;
    const y = this.mouseY;

      for (let i = 0; i < 360; i += 10)
      {
            const angle = i;
            const x1 = Math.floor(r * Math.cos(angle * Math.PI / 180));
            const y1 = Math.floor(r * Math.sin(angle * Math.PI / 180));
            const spawnX = x + x1;
            const spawnY = y + y1;

            if (spawnX < 2 || spawnX > this.gameWidth - 2 || spawnY < 2 || spawnY > this.gameHeight - 2) {
              continue;
            }

            this.sandGame.spawn(x+x1, y+y1, this.particleType);
      }
  }

  private renderLoop() {
    if (this.drawing) {
      this.draw();
    }

    this.sandGame.step();
    requestAnimationFrame(this.renderLoop);
  }

  selectType(type: number) {
    this.particleType = type as ParticleType;
  }

  particleTypeAsString(type: number): string {
    return ParticleType[type];
  }

  get particleTypes(): Array<number> {
    return Object.keys(ParticleType).filter(key => !isNaN(Number(key))).map(k => Number(k));
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped lang="scss"></style>
