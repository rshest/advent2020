from collections import deque


def parse_deck(data):
    return deque(int(line.strip()) for line in data.split('\n')[1:])


def get_score(deck):
    return sum(x * (len(deck) - i) for (i, x) in enumerate(deck))


def step(decks, recurse):
    da, db = decks
    if len(db) == 0:
        return 0
    elif len(da) == 0:
        return 1
    a, b = da.popleft(), db.popleft()
    a_wins = False

    if recurse and a <= len(da) and b <= len(db):
        da1 = deque(da[i] for i in range(a))
        db1 = deque(db[i] for i in range(b))
        a_wins = game([da1, db1], True) == 0
    else:
        a_wins = a > b

    if a_wins:
        da.append(a)
        da.append(b)
    else:
        db.append(b)
        db.append(a)
    return -1


def game(decks, recurse):
    visited = [set(), set()]
    while True:
        h1, h2 = hash(str(decks[0])), hash(str(decks[1]))
        if h1 in visited[0] or h2 in visited[1]:
            # infinite loop, Player 1 wins
            return 0
        visited[0].add(h1)
        visited[1].add(h2)

        winner = step(decks, recurse)
        if winner != -1:
            return winner


def solution():
    decks = [parse_deck(chunk.strip()) for chunk in open(
        '../data/22.txt', 'r').read().split('\n\n')]

    decks1 = [deck.copy() for deck in decks]
    winner1 = game(decks1, False)
    print(f'Answer 1: {get_score(decks1[winner1])}')

    decks2 = [deck.copy() for deck in decks]
    winner2 = game(decks2, True)
    print(f'Answer 2: {get_score(decks2[winner2])}')
