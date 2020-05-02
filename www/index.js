import { SandGame } from "sand-game-wasm";
import { memory } from "sand-game-wasm/sand_wasm_bg";
import * as PIXI from "pixi.js";

const canvas = document.getElementById("canvas");
const pre = document.getElementById("pre");
const button = document.getElementById("spawn");

const width = 512;
const height = 512;
const sandGame = SandGame.new(width, height);

const framebufferPointer = sandGame.framebuffer();
const canvasScale = 1;

let mouseX = 0;
let mouseY = 0;
let drawing = false;

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

sprite.interactive = true;
pixiApp.renderer.plugins.interaction.moveWhenInside = true;

const onTouchPointerMove = (event) => {
    mouseX = Math.floor(event.data.global.x / canvasScale);
    mouseY = Math.floor(event.data.global.y / canvasScale);
}

const startDrawing = () => {
    drawing = true;
}

const stopDrawing = () => {
    drawing = false;
}
const drawSand = () => {
    if (drawing) {
        for (let y = -6; y < 7; ++y) {
            for (let x = -6; x < 7; ++x) {
                // todo this should be done in the rust code
                const xx = mouseX + x;
                const yy = mouseY + y;

                if (xx < 0 || xx > width - 1 || yy < 0 || yy > height - 3) {
                    continue;
                }

                sandGame.spawn(xx, yy);
            }
        }
    }
}

sprite.on("touchmove", onTouchPointerMove);
sprite.on("pointermove", onTouchPointerMove);
sprite.on("touchstart", startDrawing);
sprite.on("touchend", stopDrawing);
sprite.on("pointerdown", startDrawing);
sprite.on("pointerup", stopDrawing);

sprite.setTransform(0, 0, canvasScale, canvasScale);

pixiApp.stage.addChild(sprite);

button.onclick = () => {
}

const renderLoop = () => {
    drawSand();
    sandGame.step();
    texture.update();

    requestAnimationFrame(renderLoop);
};

requestAnimationFrame(renderLoop);
