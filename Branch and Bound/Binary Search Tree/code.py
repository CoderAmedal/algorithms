# import visualization libraries {
from algorithm_visualizer import Tracer, Array1DTracer, GraphTracer, LogTracer, Randomize, Layout, VerticalLayout
# }

T = {}

elements = [5, 8, 10, 3, 1, 6, 9, 7, 2, 0, 4]  # item to be inserted

# define tracer variables {
graphTracer = GraphTracer(' BST - Elements marked red indicates the current status of tree ')
elemTracer = Array1DTracer(' Elements ')
logger = LogTracer(' Log ')
Layout.setRoot(VerticalLayout([graphTracer, elemTracer, logger]))
elemTracer.set(elements)
graphTracer.log(logger)
Tracer.delay()
# }


def bstInsert(root, element, parent=None):  # root = current node , parent = previous node
    # visualize {
    if parent is None:
        graphTracer.visit(root)
    else:
        graphTracer.visit(root, parent)
    Tracer.delay()
    # }
    treeNode = T[root]
    propName = ''
    if element < root:
        propName = 'left'
    elif element > root:
        propName = 'right'
    if propName != '':
        if propName not in treeNode:  # insert as child of root
            treeNode[propName] = element
            T[element] = {}
            # visualize {
            graphTracer.addNode(element)
            graphTracer.addEdge(root, element)
            graphTracer.select(element, root)
            Tracer.delay()
            graphTracer.deselect(element, root)
            logger.println("{} Inserted".format(element))
            # }
        else:
            bstInsert(treeNode[propName], element, root)
    # visualize {
    if parent is None:
        graphTracer.leave(root)
    else:
        graphTracer.leave(root, parent)
    Tracer.delay()
    # }


Root = elements[0]  # take first element as root
T[Root] = {}
# visualize {
graphTracer.addNode(Root)
graphTracer.layoutTree(Root, True)
logger.println("{} Inserted as root of tree ".format(Root))
# }

for i in range(1, len(elements)):
    # visualize {
    elemTracer.select(i)
    Tracer.delay()
    # }
    bstInsert(Root, elements[i])  # insert ith element
    # visualize {
    elemTracer.deselect(i)
    Tracer.delay()
    # }


def bst(item, node, parent=None):  # node = current node , parent = previous node
    # visualize {
    if parent is None:
        graphTracer.visit(node)
    else:
        graphTracer.visit(node, parent)
    Tracer.delay()
    # }
    if item == node:  # key found
        # logger {
        logger.println(' Match Found ')
        # }
    elif item < node:  # key less than value of current node
        child = T[node].get('left')
        if child is None:
            # logger {
            logger.println(' Not Found ')
            # }
        else:
            bst(item, child, node)
    else:  # key greater than value of current node
        child = T[node].get('right')
        if child is None:
            # logger {
            logger.println(' Not Found ')
            # }
        else:
            bst(item, child, node)


key = elements[Randomize.Integer(min=0, max=len(elements) - 1).create()]  # item to be searched

# logger {
logger.println("Finding number {}".format(key))
# }
bst(key, Root)  # node with key Root is the root
