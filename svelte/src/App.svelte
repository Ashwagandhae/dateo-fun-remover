<script>
  import Solution from './Solution.svelte';
  import { getFiveNums } from './nums.js';
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
      if (data.message === 'done') {
        stop();
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

  let dateGoal, dateNum1, dateNum2, dateNum3, dateNum4, dateNum5;
  $: {
    let [year, month, day] = date.split('-').map((x) => parseInt(x));
    dateGoal = day;
    [dateNum1, dateNum2, dateNum3, dateNum4, dateNum5] = getFiveNums(
      year,
      month - 1,
      day
    );
  }
</script>

<main>
  <section class="intro">
    <h1>Date-o Fun Remover!</h1>
    <p>
      A solver for the hit online game date-o. Check out the <a
        href="http://dateo-math-game.com">real game</a
      > for real fun. This solver was made in collaboration with Finn McKibbin.
    </p>
    <p>
      <a href="https://github.com/Ashwagandhae/dateo-fun-remover">Source code</a
      >
    </p>
  </section>
  <section class="input">
    <div class="inputMode">
      <button on:click={() => (useDate = true)} class:selected={useDate}
        >Date</button
      >
      <button on:click={() => (useDate = false)} class:selected={!useDate}
        >Goal & Numbers</button
      >
    </div>
    {#if useDate}
      <input type="date" bind:value={date} />
      <div>
        <span>Goal: {dateGoal}, </span>
        <span
          >Nums: {dateNum1}, {dateNum2}, {dateNum3}, {dateNum4}, {dateNum5}</span
        >
      </div>
    {:else}
      <label for="goal">Goal</label>
      <input type="number" id="goal" bind:value={goal} />
      <div class="nums">
        <div class="num">
          <label for="num1">Num1</label>
          <input type="number" id="num1" bind:value={num1} />
        </div>
        <div class="num">
          <label for="num2">Num2</label>
          <input type="number" id="num2" bind:value={num2} />
        </div>
        <div class="num">
          <label for="num3">Num3</label>
          <input type="number" id="num3" bind:value={num3} />
        </div>
        <div class="num">
          <label for="num4">Num4</label>
          <input type="number" id="num4" bind:value={num4} />
        </div>
        <div class="num">
          <label for="num5">Num5</label>
          <input type="number" id="num5" bind:value={num5} />
        </div>
      </div>
    {/if}
  </section>
  <section class="solutions">
    <div class="status">
      <button on:click={toggle} class="super" class:running
        >{running ? 'Stop' : 'Start'}</button
      >
      <div class="thinking" class:on={running} />
    </div>
    <ul class="content">
      {#each content as item}
        <li>
          <Solution {...item} />
        </li>
      {/each}
    </ul>
  </section>
</main>

<style>
  @keyframes pulse-flash {
    0% {
      background-color: var(--back-2);
    }
    50% {
      background-color: var(--back-3);
    }
    100% {
      background-color: var(--back-2);
    }
  }
  main {
    color-scheme: dark;

    display: flex;
    flex-direction: column;
    align-items: center;
    font-size: 1.2rem;
    height: auto;
    min-height: 100vh;
    width: 100vw;
    padding-top: 10vh;
    padding-bottom: 50vh;
    gap: 1rem;
  }
  section {
    display: block;
    width: 100%;
    max-width: 50ch;
    box-sizing: border-box;
  }
  .nums {
    display: flex;
    flex-direction: row;
    justify-content: space-between;
    width: 100%;
    /* allow flex to go to next line  if needed */
    flex-wrap: wrap;
  }
  input[type='number'] {
    width: 4ch;
    border: none;
    border-radius: var(--rad);
    background: var(--back-3);
    padding: 0.5rem;
    color: var(--text);
  }
  input[type='date'] {
    width: 100%;
    border: none;
    border-radius: var(--rad);
    background: var(--back-3);
    padding: 0.5rem;
    color: var(--text);
  }
  /* hide up and down arrows */
  input[type='number']::-webkit-outer-spin-button,
  input[type='number']::-webkit-inner-spin-button {
    -webkit-appearance: none;
  }
  /* hide active outline */
  input[type='number']:focus {
    background: var(--back-4);
  }

  ul {
    list-style: none;
    padding: 0;
    display: flex;
    flex-direction: column;
    gap: 1rem;
    margin: 0;
    padding-top: 1rem;
  }
  .status {
    display: flex;
    flex-direction: row;
    width: 100%;
    justify-content: space-between;
  }
  section.input {
    background: var(--back-2);
    padding: 1rem;
    border-radius: var(--rad);
  }
  button {
    background: var(--back-2);
    border: none;
    border-radius: var(--rad);
    padding: 0.5rem;
    color: var(--text);
    cursor: pointer;
  }
  button:active {
    background: var(--back-3);
  }
  .inputMode button.selected {
    background: var(--back-3);
  }
  .inputMode button.selected:active {
    background: var(--back-4);
  }
  button.super {
    font-size: 2.4rem;
    padding: 1rem;
    margin: 0;
    width: 100%;
  }
  button.super.running {
    animation: pulse-flash 1s infinite;
  }
  h1 {
    margin: 0;
    font-weight: normal;
    font-size: 3rem;
  }
</style>
