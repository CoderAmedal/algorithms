# import visualization libraries {
from algorithm_visualizer import Array2DTracer, Layout, LogTracer, GraphTracer, Tracer, VerticalLayout
# }


# define tracer variables {
class ListNode:
    def __init__(self, val):
        self.val = val
        self.next = None


node0 = ListNode(0)
node1 = ListNode(1)
node2 = ListNode(2)
node3 = ListNode(3)
node4 = ListNode(4)
node5 = ListNode(5)
node6 = ListNode(6)

list = node0
list.next = node1
list.next.next = node2
list.next.next.next = node3
list.next.next.next.next = node4
list.next.next.next.next.next = node5
list.next.next.next.next.next.next = node6
list.next.next.next.next.next.next.next = node2

graphTracer = GraphTracer("Linked List")
graphTracer.directed()
logTracer = LogTracer("Console")
Layout.setRoot(VerticalLayout([graphTracer, logTracer]))

graphTracer.addNode(node0.val)
graphTracer.addNode(node1.val)
graphTracer.addNode(node2.val)
graphTracer.addNode(node3.val)
graphTracer.addNode(node4.val)
graphTracer.addNode(node5.val)
graphTracer.addNode(node6.val)
graphTracer.addEdge(node0.val, node1.val)
graphTracer.addEdge(node1.val, node2.val)
graphTracer.addEdge(node2.val, node3.val)
graphTracer.addEdge(node3.val, node4.val)
graphTracer.addEdge(node4.val, node5.val)
graphTracer.addEdge(node5.val, node6.val)
graphTracer.addEdge(node6.val, node2.val)
Tracer.delay()
# }


def listHasCycle(head):
    # visualize {
    graphTracer.select(head.val)
    graphTracer.visit(head.val)
    Tracer.delay()
    graphTracer.deselect(head.val)
    graphTracer.leave(head.val)
    # }

    # 1. is there a cycle?
    slow = head.next
    fast = head.next.next
    # visualize {
    graphTracer.select(slow.val)
    graphTracer.visit(fast.val)
    Tracer.delay()
    graphTracer.deselect(slow.val)
    graphTracer.leave(fast.val)
    # }
    while slow is not fast:
        slow = slow.next
        fast = fast.next.next
        # visualize {
        graphTracer.select(slow.val)
        graphTracer.visit(fast.val)
        Tracer.delay()
        graphTracer.deselect(slow.val)
        graphTracer.leave(fast.val)
        # }

    # 2. where does the cycle start?
    cycleStartPosition = 0
    slow = head
    # visualize {
    graphTracer.select(slow.val)
    graphTracer.visit(fast.val)
    Tracer.delay()
    graphTracer.deselect(slow.val)
    graphTracer.leave(fast.val)
    # }
    while slow is not fast:
        slow = slow.next
        fast = fast.next
        cycleStartPosition += 1
        # visualize {
        graphTracer.select(slow.val)
        graphTracer.visit(fast.val)
        Tracer.delay()
        graphTracer.deselect(slow.val)
        graphTracer.leave(fast.val)
        # }

    # 3. what is the length of the cycle?
    cycleLength = 1
    fast = slow.next
    # visualize {
    graphTracer.select(slow.val)
    graphTracer.visit(fast.val)
    Tracer.delay()
    graphTracer.deselect(slow.val)
    graphTracer.leave(fast.val)
    # }
    while slow is not fast:
        fast = fast.next
        cycleLength += 1
        # visualize {
        graphTracer.select(slow.val)
        graphTracer.visit(fast.val)
        Tracer.delay()
        graphTracer.deselect(slow.val)
        graphTracer.leave(fast.val)
        # }

    return {
        "cycleLength": cycleLength,
        "cycleStartPosition": cycleStartPosition,
    }


# log {
res = listHasCycle(list)
logTracer.print("cycle start position: {}".format(res["cycleStartPosition"]))
logTracer.print("\n")
logTracer.print("cycle length: {}".format(res["cycleLength"]))
# }
