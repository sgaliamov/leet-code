/// Given head, the head of a linked list, determine if the linked list has a cycle in it.
/// There is a cycle in a linked list if there is some node in the list that can be reached again by continuously following the next pointer. Internally, pos is used to denote the index of the node that tail's next pointer is connected to. Note that pos is not passed as a parameter.
/// Return true if there is a cycle in the linked list. Otherwise, return false.
///
/// Constraints:
/// - The number of the nodes in the list is in the range [0, 10^4].
/// - -10^5 <= Node.val <= 10^5
/// - pos is -1 or a valid index in the linked-list.
///
/// Follow up: Can you solve it using O(1) (i.e. constant) memory?

RunTests(new Solution().HasCycle_1);
Console.WriteLine("All tests passed");
return 0;

static void RunTests(Func<ListNode?, bool> target)
{
    // Test 1: Cycle at position 1
    var node1 = new ListNode(3);
    var node2 = new ListNode(2);
    var node3 = new ListNode(0);
    var node4 = new ListNode(-4);
    node1.next = node2;
    node2.next = node3;
    node3.next = node4;
    node4.next = node2;
    AssertEqual(target(node1), true, "cycle at pos 1");

    // Test 2: Cycle at position 0
    var singleCycle1 = new ListNode(1);
    var singleCycle2 = new ListNode(2);
    singleCycle1.next = singleCycle2;
    singleCycle2.next = singleCycle1;
    AssertEqual(target(singleCycle1), true, "cycle at pos 0");

    // Test 3: No cycle, single node
    var single = new ListNode(1);
    AssertEqual(target(single), false, "no cycle, single node");

    // Test 4: No cycle, multiple nodes
    var noCycle1 = new ListNode(1);
    var noCycle2 = new ListNode(2);
    var noCycle3 = new ListNode(3);
    noCycle1.next = noCycle2;
    noCycle2.next = noCycle3;
    AssertEqual(target(noCycle1), false, "no cycle, multiple nodes");

    // Test 5: Null head
    AssertEqual(target(null), false, "null head");
}

static void AssertEqual(bool actual, bool expected, string testName)
{
    if (actual != expected)
    {
        Console.WriteLine($"FAIL: {testName} - Expected {expected}, got {actual}");
        Environment.Exit(1);
    }
}

public class Solution
{
    // 111ms - 19.54% | 48.83 - 19.79%
    public bool HasCycle_1(ListNode head)
    {
        var set = new HashSet<ListNode>();

        while (head?.next != null)
        {
            if (!set.Add(head))
            {
                return true;
            }

            head = head.next;
        }

        return false;
    }
}

public class ListNode
{
    public int val;
    public ListNode? next;

    public ListNode(int x)
    {
        val = x;
        next = null;
    }
}
