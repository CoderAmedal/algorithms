# import visualization libraries {
from algorithm_visualizer import Array1DTracer, GraphTracer, LogTracer, Tracer, Layout, VerticalLayout
# }

G = [
    [0, 0, 1, 1, 0, 0],
    [1, 0, 0, 0, 0, 0],
    [0, 1, 0, 0, 0, 0],
    [0, 0, 0, 1, 0, 0],
    [0, 0, 0, 0, 0, 1],
    [0, 0, 0, 0, 1, 0],
]

disc = [-1] * len(G)
low = [-1] * len(G)
stackMember = [False] * len(G)
st = []
carry = {'time': 0}

# define tracer variables {
graphTracer = GraphTracer()
graphTracer.set(G)
discTracer = Array1DTracer('Disc')
lowTracer = Array1DTracer('Low')
stackMemberTracer = Array1DTracer('stackMember')
stTracer = Array1DTracer('st')
logger = LogTracer()
Layout.setRoot(VerticalLayout([graphTracer, discTracer, lowTracer, stackMemberTracer, stTracer, logger]))
discTracer.set(disc)
lowTracer.set(low)
stackMemberTracer.set(stackMember)
stTracer.set(st)
Tracer.delay()
# }


def scc_vertex(u):
    # visualize {
    graphTracer.visit(u)
    Tracer.delay()
    # }

    carry['time'] += 1
    disc[u] = carry['time']
    # visualize {
    discTracer.patch(u, carry['time'])
    Tracer.delay()
    # }

    low[u] = carry['time']
    # visualize {
    lowTracer.patch(u, carry['time'])
    Tracer.delay()
    # }

    st.append(u)
    # visualize {
    stTracer.set(st)
    Tracer.delay()
    # }

    stackMember[u] = True
    # visualize {
    stackMemberTracer.patch(u, True)
    Tracer.delay()
    # }

    # Go through all vertices adjacent to this
    for v in range(len(G[u])):
        if G[u][v]:
            # If v is not visited yet, then recur for it
            if disc[v] == -1:
                scc_vertex(v)

                # Check if the subtree rooted with 'v' has a
                # connection to one of the ancestors of 'u'
                low[u] = min(low[u], low[v])
                # visualize {
                lowTracer.patch(u, low[u])
                Tracer.delay()
                # }

            # Update low value of 'u' only of 'v' is still in stack
            # (i.e. it's a back edge, not cross edge).
            elif stackMember[v] is True:
                low[u] = min(low[u], disc[v])
                # visualize {
                lowTracer.patch(u, low[u])
                Tracer.delay()
                # }

    # head node found, pop the stack and print an SCC
    w = 0  # To store stack extracted vertices
    if low[u] == disc[u]:
        while st[len(st) - 1] != u:
            w = st.pop()
            # visualize {
            stTracer.set(st)
            Tracer.delay()

            logger.println(w)
            Tracer.delay()
            # }

            stackMember[w] = False
            # visualize {
            stackMemberTracer.patch(w, False)
            Tracer.delay()
            # }

        w = st.pop()
        # visualize {
        stTracer.set(st)
        Tracer.delay()

        logger.println(w)
        Tracer.delay()
        logger.println('------')
        # }

        stackMember[w] = False
        # visualize {
        stackMemberTracer.patch(w, False)
        Tracer.delay()
        # }


for i in range(len(G)):
    if disc[i] == -1:
        scc_vertex(i)
