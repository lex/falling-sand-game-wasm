import { SandGame } from "sand-game-wasm";

const pre = document.getElementById("canvas");
const sandGame = SandGame.new();

const renderLoop = () => {
    pre.textContent = sandGame.render();
    sandGame.step();

    requestAnimationFrame(renderLoop);
};

requestAnimationFrame(renderLoop);
