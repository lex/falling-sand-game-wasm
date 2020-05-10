<template>
  <b-container fluid>
    <b-navbar ref="navbar" toggleable="lg" type="dark" variant="dark">
      <b-navbar-brand href="#">Sand Game</b-navbar-brand>

      <b-navbar-toggle target="particle-type-collapse">
        <template v-slot:default="{ expanded }">
          <b-text v-if="expanded">
            {{ particleTypeAsString(particleType) }}
          </b-text>
          <b-text v-else>
            {{ particleTypeAsString(particleType) }}
          </b-text>
        </template>
      </b-navbar-toggle>

      <b-navbar-toggle target="brush-size-collapse">
        <b-text v-if="expanded">
          {{ brushSizeAsString(brushSize) }}
        </b-text>
        <b-text v-else>
          {{ brushSizeAsString(brushSize) }}
        </b-text>
      </b-navbar-toggle>

      <b-navbar-toggle target="debug-collapse">
        <b-text v-if="expanded">
          Debug
        </b-text>
        <b-text v-else>
          Debug
        </b-text>
      </b-navbar-toggle>

      <b-collapse id="particle-type-collapse" is-nav>
        <b-navbar-nav class="ml-auto">
          <b-nav-item
            v-for="pType in particleTypes"
            :key="pType"
            :active="pType == particleType"
            v-on:click="selectType(pType)"
            >{{ particleTypeAsString(pType) }}</b-nav-item
          >
        </b-navbar-nav>
      </b-collapse>

      <b-collapse id="brush-size-collapse" is-nav>
        <b-navbar-nav class="ml-auto">
          <b-nav-item v-on:click="setBrushSize(1)">Tiny</b-nav-item>
          <b-nav-item v-on:click="setBrushSize(3)">Small</b-nav-item>
          <b-nav-item v-on:click="setBrushSize(5)">Medium</b-nav-item>
          <b-nav-item v-on:click="setBrushSize(8)">Large</b-nav-item>
          <b-nav-item v-on:click="setBrushSize(10)">Extra Large</b-nav-item>
        </b-navbar-nav>
      </b-collapse>

      <b-collapse id="debug-collapse" is-nav>
        <b-navbar-nav class="ml-auto">
          <b-nav-item v-on:click="clear">Clear</b-nav-item>
          <b-nav-item v-on:click="debugFill">Fill</b-nav-item>
          <b-nav-item v-on:click="updateScaling">Update scaling</b-nav-item>
        </b-navbar-nav>
      </b-collapse>
    </b-navbar>
    <div id="canvas-container" ref="canvascontainer">
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

  private windowWidth = window.innerWidth;
  private windowHeight = window.innerHeight;

  private _canvasWidth = 1;
  private _canvasHeight = 1;
  private canvasScaleX = 1;
  private canvasScaleY = 1;
  private canvas?: HTMLCanvasElement = undefined;

  private mouseX = 0;
  private mouseY = 0;
  private drawing = false;
  private touching = false;

  private brushSize = 5;

  private particleType = ParticleType.Sand;

  async mounted() {
    this.canvas = this.$refs.canvas as HTMLCanvasElement;
    this.setupCanvas();
    window.addEventListener("resize", this.onResize);

    await this.loadWasm();
    this.setupGame();
    this.updateScaling();

    requestAnimationFrame(this.renderLoop);
  }

  destroyed() {
    window.removeEventListener("resize", this.onResize);
  }

  private onResize() {
    this.updateScaling();
  }

  private setupCanvas() {
    this._canvasWidth = this.gameWidth;
    this._canvasHeight = this.gameHeight;
  }

  private updateScaling() {
    const canvasContainer = this.$refs.canvascontainer as HTMLElement;
    const scaleX = window.innerWidth / this.gameWidth;
    const scaleY = (window.innerHeight - canvasContainer.offsetTop) / this.gameHeight;
    this.canvasScaleX = scaleX;
    this.canvasScaleY = scaleY;
    this.sandGame.update_viewport(this.canvasWidth, this.canvasHeight)
  }

  private async loadWasm() {
    const wasm = await import("../../pkg/index");
    this.wasm = wasm;
  }

  private onMouseMove(event: MouseEvent) {
    this.mouseX = Math.floor(event.offsetX / this.canvasScaleX);
    this.mouseY = Math.floor(event.offsetY / this.canvasScaleY);
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
      const x = Math.floor((touch.pageX - xx) / this.canvasScaleX);
      const y = Math.floor((touch.pageY - yy) / this.canvasScaleY);
      this.mouseX = x;
      this.mouseY = y;
    }
  }

  get canvasWidth() {
    return this._canvasWidth * this.canvasScaleX;
  }

  get canvasHeight() {
    return this._canvasHeight * this.canvasScaleY;
  }

  private setupGame() {
    this.sandGame = this.wasm.SandGame.new(this.gameWidth, this.gameHeight);
    this.sandGame.initialize_webgl();
  }

  private draw(ox: number, oy: number) {
    const r = this.brushSize;

    for (let y = -r; y <= r; ++y) {
      for (let x = -r; x <= r; ++x) {
        if (x * x + y * y <= r * r) {
          if (ox + x < 2 || ox + x > this.gameWidth - 2 || oy + y < 1 || oy + y > this.gameHeight - 2) {
            continue;
          }

          this.sandGame.spawn(ox+x, oy+y, this.particleType);
        }
      }
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

  private clear() {
    for (let y = 1; y < this.gameHeight - 1; ++y) {
      for (let x = 1; x < this.gameWidth - 1; ++x) {
        this.sandGame.spawn(x, y, ParticleType.Empty);
      }
    }
  }

  private setBrushSize(size: number) {
    this.brushSize = size;
  }

  private brushSizeAsString(size: number) {
    switch (size) {
      case 1:
        return "Tiny";
      case 3:
        return "Small";
      case 5:
        return "Medium";
      case 8:
        return "Large";
      case 10:
        return "Extra Large";
        default:
          return "undefined";
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
.container-fluid {
  padding-left: 0px;
  padding-right: 0px;
}
</style>
