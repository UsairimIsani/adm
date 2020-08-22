class Node(object):
    def __init__(self, value):
        self.v = value
        self.l = None
        self.r = None


class BinaryTree(object):
    def __init__(self):
        self.root = None

    def get_root(self):
        return self.root

    def _insert(self, value, node):
        if value < node.v:
            if node.l is None:
                node.l = Node(value)
            else:
                self._insert(value, node.l)
        else:
            if node.r is None:
                node.r = Node(value)
            else:
                self._insert(value, node.r)

    def insert(self, value):
        if self.root is None:
            self.root = Node(value)
        else:
            self._insert(value, self.root)

    def _find(self, value, node):
        if value == node.v:
            return node
        elif value < node.v and node.l is not None:
            return self._find(value, node.l)
        elif value > node.v and node.r is not None:
            return self._find(value, node.r)

    def find(self, value):
        if self.root is None:
            return None
        return self._find(value, self.root)

    def min_value_node(self, node):
        current = node
        while(current.left is not None):
            current = current.left
        return current

    def _delete(self, node, value):
        if node is None:
            return node
        if value < node.v:
            node.l = self._delete(node.l, value)
        elif value > node.v:
            node.r = self._delete(node.r, value)
        else:
            if node.l is None:
                temp = node.r
                node = None
                return temp
            elif node.r is None:
                temp = node.l
                node = None
                return temp
            temp = self.min_value_node(node.r)
            node.v = temp.v
            node.r = self._delete(node.r, temp.v)
            return temp

    def delete(self, value):
        return self._delete(self.root, value)

    def delete_tree(self):
        self.root = None

    def _in_order(self, node):
        if node is not None:
            self._in_order(node.l)
            print("{} ".format(node.v))
            self._in_order(node.r)

    def _pre_order(self, node):
        if node is not None:
            print("{} ".format(node.v))
            self._in_order(node.l)
            self._in_order(node.r)

    def _post_order(self, node):
        if node is not None:
            self._in_order(node.l)
            self._in_order(node.r)
            print("{} ".format(node.v))

    def traverse(self, order):
        if self.root is None:
            print("Tree is Empty.")
            return
        if order == "pre":
            f = self._pre_order
        elif order == "post":
            f = self._post_order
        else:
            f = self._in_order
        return f(self.root)
# Please Comment How it works line by Line.
