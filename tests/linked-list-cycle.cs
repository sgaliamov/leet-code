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

var solution = new Solution();

// Test 1: Cycle exists
var node1 = new ListNode(3);
var node2 = new ListNode(2);
var node3 = new ListNode(0);
var node4 = new ListNode(-4);
node1.next = node2;
node2.next = node3;
node3.next = node4;
node4.next = node2; // cycle

if (!solution.HasCycle_1(node1))
{
    Console.WriteLine("FAIL: Test 1 - Expected true");
    return 1;
}

// Test 2: No cycle
var single = new ListNode(1);
if (solution.HasCycle_1(single))
{
    Console.WriteLine("FAIL: Test 2 - Expected false");
    return 1;
}

// Test 3: null
if (solution.HasCycle_1(null!))
{
    Console.WriteLine("FAIL: Test 3 - Expected false");
    return 1;
}

Console.WriteLine("All tests passed");
return 0;

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
