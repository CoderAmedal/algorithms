import org.algorithm_visualizer.*;

class Main {

    private static GraphTracer graphTracer = new GraphTracer("Linked List");

    private static LogTracer logTracer = new LogTracer("Console");

    private static class ListNode {
        int val;
        ListNode next;

        ListNode(int val) {
            this.val = val;
            this.next = null;
        }
    }

    public static void main(String[] args) {
        ListNode node0 = new ListNode(0);
        ListNode node1 = new ListNode(1);
        ListNode node2 = new ListNode(2);
        ListNode node3 = new ListNode(3);
        ListNode node4 = new ListNode(4);
        ListNode node5 = new ListNode(5);
        ListNode node6 = new ListNode(6);

        ListNode list = node0;
        list.next = node1;
        list.next.next = node2;
        list.next.next.next = node3;
        list.next.next.next.next = node4;
        list.next.next.next.next.next = node5;
        list.next.next.next.next.next.next = node6;
        list.next.next.next.next.next.next.next = node2;

        graphTracer.directed();
        Layout.setRoot(new VerticalLayout(new Commander[]{graphTracer, logTracer}));

        graphTracer.addNode(node0.val);
        graphTracer.addNode(node1.val);
        graphTracer.addNode(node2.val);
        graphTracer.addNode(node3.val);
        graphTracer.addNode(node4.val);
        graphTracer.addNode(node5.val);
        graphTracer.addNode(node6.val);
        graphTracer.addEdge(node0.val, node1.val);
        graphTracer.addEdge(node1.val, node2.val);
        graphTracer.addEdge(node2.val, node3.val);
        graphTracer.addEdge(node3.val, node4.val);
        graphTracer.addEdge(node4.val, node5.val);
        graphTracer.addEdge(node5.val, node6.val);
        graphTracer.addEdge(node6.val, node2.val);
        Tracer.delay();

        int[] res = listHasCycle(list);
        logTracer.print("cycle start position: " + res[1]);
        logTracer.print("\n");
        logTracer.print("cycle length: " + res[0]);
    }

    private static int[] listHasCycle(ListNode head) {
        graphTracer.select(head.val);
        graphTracer.visit(head.val);
        Tracer.delay();
        graphTracer.deselect(head.val);
        graphTracer.leave(head.val);

        // 1. is there a cycle?
        ListNode slow = head.next;
        ListNode fast = head.next.next;
        graphTracer.select(slow.val);
        graphTracer.visit(fast.val);
        Tracer.delay();
        graphTracer.deselect(slow.val);
        graphTracer.leave(fast.val);
        while (slow != fast) {
            slow = slow.next;
            fast = fast.next.next;
            graphTracer.select(slow.val);
            graphTracer.visit(fast.val);
            Tracer.delay();
            graphTracer.deselect(slow.val);
            graphTracer.leave(fast.val);
        }

        // 2. where does the cycle start?
        int cycleStartPosition = 0;
        slow = head;
        graphTracer.select(slow.val);
        graphTracer.visit(fast.val);
        Tracer.delay();
        graphTracer.deselect(slow.val);
        graphTracer.leave(fast.val);
        while (slow != fast) {
            slow = slow.next;
            fast = fast.next;
            cycleStartPosition += 1;
            graphTracer.select(slow.val);
            graphTracer.visit(fast.val);
            Tracer.delay();
            graphTracer.deselect(slow.val);
            graphTracer.leave(fast.val);
        }

        // 3. what is the length of the cycle?
        int cycleLength = 1;
        fast = slow.next;
        graphTracer.select(slow.val);
        graphTracer.visit(fast.val);
        Tracer.delay();
        graphTracer.deselect(slow.val);
        graphTracer.leave(fast.val);
        while (slow != fast) {
            fast = fast.next;
            cycleLength += 1;
            graphTracer.select(slow.val);
            graphTracer.visit(fast.val);
            Tracer.delay();
            graphTracer.deselect(slow.val);
            graphTracer.leave(fast.val);
        }

        return new int[]{cycleLength, cycleStartPosition};
    }
}
