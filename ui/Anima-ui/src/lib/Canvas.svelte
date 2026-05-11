<script lang="ts">
    import { onMount } from "svelte";
    import init, { AnimaCanvas } from "$AnimaCanvas/Anima_canvas.js";

    let canvasElement: HTMLCanvasElement;
    let anima: AnimaCanvas;

    onMount(async () => {
        try {
            await init();

            anima = AnimaCanvas.new_with_element(canvasElement);

            anima.draw();

            console.log("AnimaCanvas inicializado con éxito");
        } catch (error) {
            console.error("Error al inicializar WASM:", error);
        }
    });
</script>

<div class="container">
    <canvas bind:this={canvasElement}></canvas>
</div>

<style>
    .container {
        display: flex;
        justify-content: center;
        background-color: #1a1a1a;
        padding: 1rem;
    }

    canvas {
        background: white;
        border-radius: 4px;
        width: 800px;
        height: 600px;
    }
</style>
