// import visualization libraries {
const { Tracer, Array1DTracer, LogTracer, Randomize, Layout, VerticalLayout } = require('algorithm-visualizer');
// }

// define tracer variables {
const array1dTracer = new Array1DTracer('Set');
const logTracer = new LogTracer('Console');
// }

// define input variables
const n = 10;
let s;
let d;

function solve() {
  const sel = new Array(n + 1).fill(0);
  let k = 0;
  let sum = 0;
  let found = 0;
  sel[0] = 1;
  array1dTracer.select(k);
  Tracer.delay();
  while (true) {
    if (k < n && sel[k] === 1) {
      if (sum + s[k] === d) {
        found = 1;
        logTracer.print('{');
        for (let i = 0; i < n; i++) {
          if (sel[i] === 1) {
            logTracer.print(`${s[i]}  `);
          }
        }
        logTracer.println('}');
        sel[k] = 0;
        Tracer.delay();
        array1dTracer.deselect(k);
        Tracer.delay();
      } else if (sum + s[k] < d) {
        sum += s[k];
      } else {
        sel[k] = 0;
        array1dTracer.deselect(k);
        Tracer.delay();
      }
    } else {
      k--;
      while (k >= 0 && sel[k] === 0) {
        k--;
      }
      if (k < 0) {
        break;
      }
      sel[k] = 0;
      array1dTracer.deselect(k);
      Tracer.delay();
      sum -= s[k];
    }
    k++;
    if (k < n) {
      sel[k] = 1;
      array1dTracer.select(k);
      Tracer.delay();
    }
  }
  if (found === 0) {
    logTracer.println('Not possible subsets');
  }
}

// visualize {
Layout.setRoot(new VerticalLayout([array1dTracer, logTracer]));
Tracer.delay();
// }
// Randomizing the array{
s = Randomize.Array1D({ N: n, value: () => Randomize.Integer({ min: 0, max: 29 }) });
d = Randomize.Integer({ min: 0, max: 99 });
// }

logTracer.print('The Given set is: ');
for (const x of s) {
  logTracer.print(`${x},`);
}
logTracer.println(`\nDesired sum is:${d}`);
logTracer.println(`The possible subsets of sum ${d} are: `);
array1dTracer.set(s);
Tracer.delay();
solve();
