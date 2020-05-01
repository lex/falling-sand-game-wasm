import { SandGame } from "sand-game-wasm";

const pre = document.getElementById("canvas");
const button = document.getElementById("spawn");
const sandGame = SandGame.new();

button.onclick = () => {
    sandGame.spawn();
}

const renderLoop = () => {
    pre.textContent = sandGame.render();
    sandGame.step();

    requestAnimationFrame(renderLoop);
};

requestAnimationFrame(renderLoop);
