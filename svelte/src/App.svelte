<script>
  import { onMount } from 'svelte';

  let worker;
  let content = [];

  let running = false;

  function start() {
    content = [];
    running = true;
    worker = new Worker('./build/worker.js');
    worker.onmessage = ({ data }) => {
      if (data.message === 'ready') {
        worker.postMessage({
          message: 'start',
          useDate,
          goal,
          num1,
          num2,
          num3,
          num4,
          num5,
          date,
        });
      }
      if (data.message === 'solution') {
        content = [...content, data.solution];
      }
    };
  }
  function stop() {
    running = false;
    worker.terminate();
  }
  function toggle() {
    if (running) {
      stop();
    } else {
      start();
    }
  }

  // Budget rust enum
  let useDate = true;

  let goal = 1;
  let [num1, num2, num3, num4, num5] = [1, 2, 3, 4, 5];

  let date = new Date().toISOString().slice(0, 10);
</script>

<h1>Dateo Fun Remover!</h1>
<p>
  Check out the <a href="http://dateo-math-game.com">for real fun.</a> This solver
  was made in collaboration with Finn McKibbin.
</p>
<p>
  <a href="https://github.com/Ashwagandhae/dateo-fun-remover">Source code</a>
</p>
<p />
<div>
  <button on:click={toggle}>{running ? 'Stop' : 'Start'}</button>
  <label for="useDate">Use Date</label>
  <input type="checkbox" bind:checked={useDate} id="useDate" />
  {#if useDate}
    <input type="date" bind:value={date} />
  {:else}
    <label for="goal">Goal</label>
    <input type="number" id="goal" bind:value={goal} />
    <label for="num1">Num1</label>
    <input type="number" id="num1" bind:value={num1} />
    <label for="num2">Num2</label>
    <input type="number" id="num2" bind:value={num2} />
    <label for="num3">Num3</label>
    <input type="number" id="num3" bind:value={num3} />
    <label for="num4">Num4</label>
    <input type="number" id="num4" bind:value={num4} />
    <label for="num5">Num5</label>
    <input type="number" id="num5" bind:value={num5} />
  {/if}
  <ul class="content">
    {#each content as item}
      <li>{item}</li>
    {/each}
  </ul>
</div>
