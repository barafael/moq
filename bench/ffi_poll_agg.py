import json, re, statistics, sys
from collections import defaultdict
vals = defaultdict(list)
for line in open('.spike/go.txt'):
    p = line.split()
    if len(p) >= 4 and p[0] == 'SPIKE' and not p[1].startswith('cancel'):
        vals[('go',) + tuple(p[1].split('/', 1))].append(float(p[-1]))
for r in json.load(open('.spike/py.json')):
    vals[('py',) + tuple(r['name'].split('/', 1))].append(r['value'])
med = {k: statistics.median(v) for k, v in vals.items()}
def show(lang):
    print(f"\n## {lang}")
    calls = sorted(k for k in med if k[0] == lang and k[1] == 'call')
    print("ready read_frame ns/call:", ", ".join(f"{k[2].split('/')[0]}={med[k]:.0f}" for k in calls))
    keys = sorted({k[2] for k in med if k[0] == lang and k[1] == 'throughput'})
    # group by mode/sub
    rows = defaultdict(dict)
    for k in keys:
        shape, rest = k.split('/', 1)
        rows[rest][shape] = k
    shapes = ['task', 'task+bridge', 'a', 'b', 'b1'] if lang == 'go' else ['task', 'a', 'b', 'b1']
    metrics = ['throughput', 'cpu', 'polls'] + (['loop-wakeups'] if lang == 'py' else [])
    print("| case | " + " | ".join(f"{m} {s}" for m in metrics for s in shapes) + " |")
    def order(r):
        mode, size, sub = r.split('/')
        return (mode, int(sub.split('=')[1]))
    for rest in sorted(rows, key=order):
        cells = []
        for m in metrics:
            for s in shapes:
                k = (lang, m, rows[rest].get(s))
                cells.append(f"{med[k]:.2f}" if k in med else "-")
        print(f"| {rest} | " + " | ".join(cells) + " |")
show('go'); show('py')
