import wasm from '../../rust/Cargo.toml';

export async function getBindings() {
  return await wasm();
}
self.doneSolving = function () {
  postMessage({ message: 'done' });
};
self.sendNextSolution = function (score, atom) {
  postMessage({
    message: 'solution',
    solution: {
      score,
      atom,
    },
  });
};
getBindings().then((bindings) => {
  postMessage({ message: 'ready' });
  let { solve_with_date, solve_with_goal_and_nums } = bindings;
  onmessage = ({ data }) => {
    if (data.message === 'start') {
      if (data.useDate) {
        let [year, month, day] = data.date.split('-');
        solve_with_date(year, month - 1, day);
      } else {
        solve_with_goal_and_nums(
          data.goal,
          data.num1,
          data.num2,
          data.num3,
          data.num4,
          data.num5
        );
      }
    }
  };
});
