//A Hamiltonian cycle is a cycle in an undirected or directed graph that visits each vertex exactly once.
// import visualization libraries {
const { Tracer, GraphTracer, LogTracer, Randomize, Layout, VerticalLayout } = require('algorithm-visualizer');
// }

// define tracer variables {
const graphTracer = new GraphTracer('GraphTracer');
const logTracer = new LogTracer('Console');
// }

const n = 8;
const x = new Array(n).fill(0);
const vis = new Array(n).fill(0);
let found = 0;
const adjacencyMatrix = [];
for (let i = 0; i < n; i++) {
  adjacencyMatrix[i] = new Array(n).fill(0);
}

function ham(k) {
  while (true) {
    nextVal(k);
    if (x[k] === -1) {
      return;
    }
    if (k === n - 1) {
      graphTracer.visit(x[0], x[k]);
      Tracer.delay();
      found = 1;
      //Printint the cycle{
      for (let i = 0; i < n; i++) {
        logTracer.print(`${x[i]}  `);
      }
      logTracer.println(0);
      //}
      graphTracer.leave(x[0], x[k]);
    } else {
      ham(k + 1);
    }
  }
}

function nextVal(k) {
  while (true) {
    let i = 0;
    if (vis[k] === 1) {
      graphTracer.leave(x[k], x[k - 1]);
    }
    vis[k] = 0;
    x[k] = (x[k] + 1) % (n + 1);
    if (x[k] === n) {
      x[k] = -1;
      return;
    }
    graphTracer.visit(x[k], x[k - 1]);
    Tracer.delay();
    vis[k] = 1;
    if (adjacencyMatrix[x[k - 1]][x[k]] === 1) {
      for (i = 0; i < k; i++) {
        if (x[i] === x[k]) {
          break;
        }
      }
      if (i === k) {
        if (k < n - 1 || (k === n - 1 && adjacencyMatrix[x[k]][x[0]] === 1)) {
          return;
        }
      }
    }
  }
}

// initializing{
for (let i = 1; i < n; i++) {
  x[i] = -1;
}
// }

// Randomizing adjacancy matrix and displaying on log screen{
logTracer.println('The adjacancy matrix is');
for (let i = 0; i < n; i++) {
  for (let j = 0; j < n; j++) {
    adjacencyMatrix[i][j] = Randomize.Integer({ min: 0, max: 1 });
    logTracer.print(`${adjacencyMatrix[i][j]}  `);
  }
  logTracer.println('');
}
// }

// visualize {
Layout.setRoot(new VerticalLayout([graphTracer, logTracer]));
graphTracer.set(adjacencyMatrix);
// }

logTracer.println('The possible solutions are');
ham(1);
if (found === 0) {
  logTracer.println('No cycles are found Try with a different graph ');
}
