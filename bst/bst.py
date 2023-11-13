import random

class Node:
    def __init__(self, data):
        self.data = data
        self.left = None
        self.right = None

def maxDepth(root):
    s = [root]
    height = 0

    while True:
        if len(s) == 0:
            return height
       
        height += 1

        count = len(s)
        while(count > 0):
            node = s.pop(0)
            if node.left:
                s.append(node.left)
            if node.right:
                s.append(node.right)
            count -= 1

def createTree(numItems, root):
    count = 0
    s = [root]
    while count < numItems:
        node = s.pop()
        if random.randint(0, 1000) > 500:
            node.left = Node(7)
            count += 1
        if random.randint(0, 1000) < 500:
            node.right = Node(7)
            count += 1

        if not (node.left or node.right):
            node.left = Node(7)
            count += 1

        if node.right:
            s.append(node.right)
        if node.left:
            s.append(node.left)

i = 0
arr = []
while i < 1000:
    root = Node(7)
    createTree(100000, root)

    n = maxDepth(root)
    arr.append(n)
    i += 1

print(max(arr))
print(min(arr))
print(sum(arr) / len(arr))
