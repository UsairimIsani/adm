word = ''


def is_first(c, order):
    copy = order.copy()
    del copy[c]
    for val in copy:
        if c in copy[val]:
            return False
    return True


def append_child(c, order):
    global word
    if c in order:
        word += order[c]
        append_child(order[c], order)


def create(str_list):
    global word
    order = {}
    for o in str_list:
        (g, l) = find(o)
        order[g] = l

    for c in order:
        if is_first(c, order):
            word += c
            break
    append_child(word[0], order)
    print(word)


def find(s):
    f = s.split('>')
    return (f[0], f[1])


create(["P>A", "I>N", "A>I", "S>P"])
