<template>
  <b-container fluid>
    <div>
      <canvas
        ref="canvas"
        id="canvas"
        :width="canvasWidth"
        :height="canvasHeight"
        v-on:mousemove="onMouseMove"
        v-on:mousedown="onMouseDown"
        v-on:mouseup="onMouseUp"
        v-on:touchstart="onTouchStart"
        v-on:touchend="onTouchEnd"
        v-on:touchmove="onTouchMove"
        >rip</canvas
      >
    </div>
    <div>
      <b-button-group size="lg">
        <b-button
          v-for="pType in particleTypes"
          :key="pType"
          :variant="pType == particleType ? 'success' : ''"
          v-on:click="selectType(pType)"
        >
          {{ particleTypeAsString(pType) }}
        </b-button>
        <b-button v-on:click="debugFill">debug fill</b-button>
      </b-button-group>
    </div>
  </b-container>
</template>

<script lang="ts">
import { Component, Vue } from "vue-property-decorator";
import type { SandGame } from "../../pkg/index";

enum ParticleType {
    Empty = 0,
    Wall = 1,
    Sand = 2,
    Water = 3,
    Plant = 4,
    Fire = 5,
}

@Component
export default class Game extends Vue {
  //eslint-disable-next-line
  private wasm!: any;
  private sandGame!: SandGame;

  private gameWidth = 256;
  private gameHeight = 256;

  private canvasScale = 2;
  private canvas?: HTMLCanvasElement = undefined;

  private mouseX = 0;
  private mouseY = 0;
  private drawing = false;

  private touching = false;

  private particleType = ParticleType.Sand;

  async mounted() {
    this.canvas = this.$refs.canvas as HTMLCanvasElement;

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

  private onTouchStart(event: TouchEvent) {
    event.preventDefault();
    this.onTouchMove(event);
    this.drawing = true;
  }

  private onTouchEnd(event: TouchEvent) {
    event.preventDefault();
    this.drawing = false;
  }

  private onTouchCancel(event: TouchEvent) {
    event.preventDefault();
    this.drawing = false;
  }

  private onTouchMove(event: TouchEvent) {
    event.preventDefault();

    const xx = this.canvas?.getBoundingClientRect().left ?? 0;
    const yy = this.canvas?.getBoundingClientRect().top ?? 0;

    for (const touch of event.changedTouches) {
      const x = Math.floor((touch.pageX - xx) / this.canvasScale);
      const y = Math.floor((touch.pageY - yy) / this.canvasScale);
      this.mouseX = x;
      this.mouseY = y;
    }
  }

  get canvasWidth() {
    return this.gameWidth * this.canvasScale;
  }
  get canvasHeight() {
    return this.gameHeight * this.canvasScale;
  }

  private setupGame() {
    this.sandGame = this.wasm.SandGame.new(this.gameWidth, this.gameHeight);
    this.sandGame.initialize_webgl();
  }

  private draw(x: number, y: number) {
    const r = 10;

    for (let i = 0; i < 360; i += 10) {
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
      this.draw(this.mouseX, this.mouseY);
    }

    this.sandGame.step();
    this.sandGame.render();

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

  private debugFill() {
    for (let y = 1; y < this.gameHeight - 1; ++y) {
      for (let x = 1; x < this.gameWidth - 1; ++x) {
        const type = ~~(Math.random() * 2) == 0 ? ParticleType.Sand : ParticleType.Water;
        this.sandGame.spawn(x, y, type);
      }
    }
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped lang="scss">
.btn:focus {
  outline: none;
  box-shadow: none;
}
</style>
