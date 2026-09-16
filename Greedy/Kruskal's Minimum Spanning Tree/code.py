# import visualization libraries {
from algorithm_visualizer import GraphTracer, LogTracer, Randomize, Tracer, Layout, VerticalLayout
# }

# define tracer variables {
tracer = GraphTracer()
tracer.directed(False)
tracer.weighted()
logger = LogTracer()
Layout.setRoot(VerticalLayout([tracer, logger]))
G = Randomize.Graph(N=5, ratio=1).directed(False).weighted(True).create()
tracer.set(G)
Tracer.delay()
# }


def kruskal():
    vcount = len(G)

    # Preprocess: sort edges by weight.
    edges = []
    for vi in range(vcount - 1):
        for vj in range(vi + 1, vcount):
            edges.append({
                '0': vi,
                '1': vj,
                'weight': G[vi][vj],
            })
    edges.sort(key=lambda edge: edge['weight'])

    # Give each vertex a tree to decide if they are already in the same tree.
    t = [{} for _ in range(vcount)]
    for i in range(vcount):
        t[i][i] = True

    wsum = 0
    n = 0
    while n < vcount - 1 and len(edges) > 0:
        e = edges.pop(0)  # Get the edge of min weight
        # visualize {
        tracer.visit(e['0'], e['1'])
        Tracer.delay()
        # }
        if t[e['0']] is t[e['1']]:
            # e[0] & e[1] already in the same tree, ignore
            # visualize {
            tracer.leave(e['0'], e['1'])
            Tracer.delay()
            # }
            continue

        # Choose the current edge.
        wsum += e['weight']

        # Merge tree of e[0] & e[1]
        tmerged = {}
        for i in t[e['0']]:
            tmerged[i] = True
        for i in t[e['1']]:
            tmerged[i] = True
        for i in tmerged:
            t[i] = tmerged

        n += 1

    # logger {
    logger.println("The sum of all edges is: {}".format(wsum))
    # }


kruskal()
