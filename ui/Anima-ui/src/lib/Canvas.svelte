<script lang="ts">
    import { onMount } from "svelte";
    import init, { Anima } from "$AnimaCore/anima_core.js";

    let canvasElement: HTMLCanvasElement;
    let anima: Anima;

    onMount(async () => {
        try {
            await init();

            anima = Anima.new_with_element(canvasElement);

            anima.draw();

        } catch (error) {

            console.error(error);
            
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
