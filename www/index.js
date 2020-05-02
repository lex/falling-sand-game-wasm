import { SandGame } from "sand-game-wasm";
import { memory } from "sand-game-wasm/sand_wasm_bg";
import * as PIXI from "pixi.js";

const canvas = document.getElementById("canvas");
const pre = document.getElementById("pre");
const button = document.getElementById("spawn");

const width = 256;
const height = 256;
const sandGame = SandGame.new(width, height);

const framebufferPointer = sandGame.framebuffer();
const canvasScale = 1;

const pixiApp = new PIXI.Application({
    width: width * canvasScale,
    height: height * canvasScale,
    view: canvas
});

let fb = new Uint8Array(memory.buffer, framebufferPointer, width * height * 3);

const texture = PIXI.Texture.fromBuffer(
    fb,
    width,
    height,
    {
      format: PIXI.FORMATS.RGB,
      type: PIXI.TYPES.UNSIGNED_BYTE
    }
  );

const sprite = PIXI.Sprite.from(texture);

sprite.setTransform(0, 0, canvasScale, canvasScale);

pixiApp.stage.addChild(sprite);

button.onclick = () => {
    sandGame.spawn();
}

const renderLoop = () => {
    sandGame.step();
    texture.update();

    requestAnimationFrame(renderLoop);
};

requestAnimationFrame(renderLoop);
