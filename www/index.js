import * as wasm from "../pkg";

initializeCanvas();

document.querySelector('#rom_selector').addEventListener('change', function() {

    const reader = new FileReader();
    reader.onload = function() {
        let arrayBuffer, array;
        arrayBuffer = this.result;
        array = new Uint8Array(arrayBuffer);

        document.querySelector('#rom_selector').setAttribute("hidden", "true")
        wasm.run_emu(array);

    }
    reader.readAsArrayBuffer(this.files[0]);

}, false);

function initializeCanvas() {
    const canvas = document.getElementById("display");
    const ctx = canvas.getContext("2d");
    ctx.fillStyle = "white";
    ctx.fillRect(0, 0, 64, 32);
}