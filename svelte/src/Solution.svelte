<script>
  export let atom;
  export let score;

  // function extractScore:
  // returns {
  //   score: number,
  //   factors: {
  //     string: number,
  //   }
  // }
  // extract points and details from score
  // it is formatted like this:
  // score (factor: points, factor2: points2, ...)
  // e.g.
  // 32 (n: 5, o: 2, f: 9)
  function extractScore(scoreString) {
    let [score, factorString] = scoreString.split(' (');
    factorString = factorString.replace(')', '');
    let factors = {};
    factorString.split(', ').forEach((factor) => {
      let [factorName, factorPoints] = factor.split(': ');
      factors[factorName] = factorPoints;
    });
    return { score, factors };
  }
  let parsedScore = extractScore(score);
</script>

<div class="solution">
  <div class="score">
    <div class="mainScore">{parsedScore.score}</div>
    <div class="factors">
      {#each Object.entries(parsedScore.factors) as [factor, points], index}
        {#if index !== 0}{' + '}{/if}<span class="factor">{points}{factor}</span
        >
      {/each}
    </div>
  </div>
  <div class="atom">{atom}</div>
</div>

<style>
  div.solution {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 0.5rem;
  }
  .score {
    background: var(--back-2);
    border-radius: var(--rad);
    padding: 0.5rem;
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
  }
  .mainScore {
    font-size: 2rem;
  }
  .factors {
    gap: 0.5rem;
    font-size: 0.8rem;
    white-space: nowrap;
  }
  .atom {
    /* allow word break */
    word-break: break-all;
  }
</style>
