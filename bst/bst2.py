import random
from tqdm import tqdm

class Node:
    def __init__(self, data):
        self.data = data
        self.left = None
        self.right = None
    
    def add_item(self, item):
        self.add_node(Node(item))
    
    def add_node(self, node):
        if node.data < self.data:
            if self.left is not None:
                self.left.add_node(node)
            else:
                self.left = node
        else:
            if self.right is not None:
                self.right.add_node(node)
            else:
                self.right = node

def maxDepth(root):
    if root is None:
        return 0
    return max(maxDepth(root.left), maxDepth(root.right)) + 1

def createTree(numItems, gen=None):
    if gen is None:
        gen = lambda: random.randint(0, 1_000_000)
    
    root = Node(gen())
    numItems -= 1

    for _ in range(numItems):
        root.add_item(gen())
    
    return root

arr = []
for i in tqdm(range(1000)):
    root = createTree(100000)

    n = maxDepth(root)
    arr.append(n)

print(max(arr))
print(min(arr))
print(sum(arr) / len(arr))

with open("output.csv", "w") as file:
    file.writelines([f"{x}\n" for x in arr])
