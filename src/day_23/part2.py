from collections import defaultdict
import itertools


lines = open("sample.txt").readlines()
connections = [tuple(line.strip().split("-")) for line in lines]
adjs = defaultdict(set)
networks = {v: [{v}] for v in set(itertools.chain(*connections))}
for v1, v2 in connections:
    adjs[v1].add(v2)
    adjs[v2].add(v1)
    for network in networks[v2]:
        if network < adjs[v1]:
            for v in (new_network := network | {v1}):
                networks[v].append(new_network)
PART_2 = ','.join(sorted(max(itertools.chain.from_iterable(networks.values()), key=len)))
print(PART_2)